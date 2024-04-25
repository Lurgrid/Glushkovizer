//! Module permettant la gestion d'expression régulière. On peut crée une
//! expression régulière "à la main" mais aussi la "parse" à partir d'une chaine
//! de caractère qui contient les opérations suivante:
//!
//! - ```a```: Où "a" est une lettre de l'alphabet entre 'a' et 'z' ou 'A' et
//!     'Z'.
//!
//! - ```$```: Caractère qui permet de représenter epsilon (le mot vide).
//!
//! - ```expr*```: Permet une répétion infinie de fois, avec une répétion de
//!     zero fois inclu.
//!
//! - ```expr.expr```: Représente la concaténation des deux expressions
//!     régulières.
//!
//! - ```expr+expr```: Permet de représenter le "ou" entre les deux expressions
//!     régulières
//!
//! # Exemple
//!
//! ```rust
//! use glushkovizer::regexp::RegExp;
//!
//! fn main() {
//!     let a = RegExp::try_from("(a+b)*.a.b+$");
//!     if let Err(s) = a {
//!         eprintln!("Error ! {}", s);
//!         return;
//!     }
//!     let a = a.unwrap();
//! }
//! ```

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use std::fmt::{self, Debug, Display, Formatter};

lrlex_mod!("./lib/reg.l");
lrpar_mod!("./lib/reg.y");

#[derive(Debug, PartialEq)]
/// Nom d'un "enum" ayant pour but de représenter une expression régulière à
///     l'aide d'un arbre, composer de symbole de type T.
pub enum RegExp<T> {
    /// Element de l'enum pour representer le mot vide epsilon.
    Epsilon,
    /// Element de l'enum pour representer une lettre de l'alphabet Sigma où
    ///     chaque élément est de type T.
    Symbol(T),
    /// Element de l'enum pour representer la répétition d'une expression
    ///     régulière. Cette répétition est infini et inclu le mot vide.
    Times(Box<RegExp<T>>),
    /// Element de l'enum pour representer la concaténation de deux sous
    ///     expression régulière.
    Concat(Box<RegExp<T>>, Box<RegExp<T>>),
    /// Element de l'enum pour representer l'union de deux sous expression
    ///     régulière.
    Or(Box<RegExp<T>>, Box<RegExp<T>>),
}

impl TryFrom<&str> for RegExp<char> {
    type Error = String;

    fn try_from(regexp: &str) -> Result<RegExp<char>, Self::Error> {
        let lexerdef = reg_l::lexerdef();
        let lexer = lexerdef.lexer(regexp);
        let (res, errs) = reg_y::parse(&lexer);
        let mut err = String::new();
        for e in &errs {
            err.push_str(&format!("{}\n", e.pp(&lexer, &reg_y::token_epp)));
        }
        match res {
            Some(Ok(r)) if errs.is_empty() => Ok(r),
            _ => {
                err.push_str("Unable to evaluate expression.");
                Err(err)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Structure ayant pour but de representer des symboles numérotés
pub(crate) struct Numbered<T>(pub(crate) T, pub(crate) usize);

#[derive(PartialEq, Debug)]
/// Structure regroupant toute les informations d'une expression régulière
pub(crate) struct GlushInfo<T> {
    pub(crate) firsts: Vec<Numbered<T>>,
    pub(crate) lasts: Vec<Numbered<T>>,
    pub(crate) null: bool,
    pub(crate) follows: Vec<(Numbered<T>, Vec<Numbered<T>>)>,
}

impl<T: Clone + Copy + PartialEq> RegExp<T> {
    /// Crée à partir d'une expression régulière une autre expression
    ///     régulière où chaque symbole sera numéroté en partant de "start" et
    ///     renvoie celle-ci dans un couple, accompagné de "start" + le nombre
    ///     de symbole numéroté et renvoie les information de l'expression
    ///     régulière.
    pub(crate) fn linearization(&self, start: usize) -> (RegExp<Numbered<T>>, usize, GlushInfo<T>) {
        match self {
            RegExp::Epsilon => (
                RegExp::Epsilon,
                start,
                GlushInfo {
                    firsts: vec![],
                    lasts: vec![],
                    null: true,
                    follows: vec![],
                },
            ),
            RegExp::Symbol(v) => {
                let s = Numbered(*v, start);
                let r = RegExp::Symbol(s);
                let end = start + 1;
                (
                    r,
                    end,
                    GlushInfo {
                        firsts: vec![s],
                        lasts: vec![s],
                        null: false,
                        follows: vec![(s, vec![])],
                    },
                )
            }
            RegExp::Times(c) => {
                let (nc, end, mut gi) = c.linearization(start);
                gi.null = true;
                for last in gi.lasts.iter() {
                    if let Some(f) = gi.follows.iter_mut().find(|f| f.0 == *last) {
                        f.1.append(&mut gi.firsts.clone());
                    }
                }
                (RegExp::Times(Box::new(nc)), end, gi)
            }
            RegExp::Or(l, r) => {
                let (nl, end, mut gil) = l.linearization(start);
                let (nr, end, mut gir) = r.linearization(end);
                gil.firsts.append(&mut gir.firsts);
                gil.lasts.append(&mut gir.lasts);
                gil.null = gil.null || gir.null;
                gil.follows.append(&mut gir.follows);
                (RegExp::Or(Box::new(nl), Box::new(nr)), end, gil)
            }
            RegExp::Concat(l, r) => {
                let (nl, end, mut gil) = l.linearization(start);
                let (nr, end, mut gir) = r.linearization(end);
                for last in gil.lasts.iter() {
                    if let Some(f) = gil.follows.iter_mut().find(|f| f.0 == *last) {
                        f.1.append(&mut gir.firsts.clone());
                    }
                }
                gil.follows.append(&mut gir.follows);
                if gil.null {
                    gil.firsts.append(&mut gir.firsts);
                }
                if gir.null {
                    gil.lasts.append(&mut gir.lasts);
                } else {
                    gil.lasts = gir.lasts;
                }
                gil.null = gil.null && gir.null;
                (RegExp::Concat(Box::new(nl), Box::new(nr)), end, gil)
            }
        }
    }
}

impl<T: Display> Display for RegExp<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Epsilon => write!(f, "$"),
            Self::Symbol(s) => write!(f, "{}", s),
            Self::Times(r) => write!(f, "{}*", r),
            Self::Or(r, l) => write!(f, "({}+{})", r, l),
            Self::Concat(r, l) => write!(f, "({}.{})", r, l),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::regexp::{GlushInfo, Numbered};

    use super::RegExp;

    #[test]
    fn epsilon() {
        let a = RegExp::try_from("$");
        assert_eq!("Ok(Epsilon)", format!("{:?}", a));
    }

    #[test]
    fn symbol() {
        let a = RegExp::try_from("a");
        assert_eq!("Ok(Symbol('a'))", format!("{:?}", a));
    }

    #[test]
    fn regex1() {
        let a = RegExp::try_from("(a+b)*.a.b+$");
        assert_eq!(
            "Ok(Or(Concat(Concat(Times(Or(Symbol('a'), Symbol('b'))), \
            Symbol('a')), Symbol('b')), Epsilon))",
            format!("{:?}", a)
        );
    }

    #[test]
    fn regex2() {
        let a = RegExp::try_from("(a + b) . ( a* . b)");
        assert_eq!(
            "Ok(Concat(Or(Symbol('a'), Symbol('b')), \
            Concat(Times(Symbol('a')), Symbol('b'))))",
            format!("{:?}", a)
        );
    }

    #[test]
    fn error_syn() {
        let a = RegExp::try_from("a....b");
        assert!(a.is_err())
    }

    #[test]
    fn error_token() {
        let a = RegExp::try_from("a.b/b");
        assert!(a.is_err())
    }

    #[test]
    fn numbered() {
        let a = RegExp::try_from("(a+b).(a*.b)");
        assert!(a.is_ok());
        let (a, n, info) = a.unwrap().linearization(1);
        assert_eq!(
            "Concat(Or(Symbol(Numbered('a', 1)), Symbol(Numbered('b', 2))), \
            Concat(Times(Symbol(Numbered('a', 3))), Symbol(Numbered('b', 4))))",
            format!("{:?}", a)
        );
        assert_eq!(5, n);
        assert_eq!(
            GlushInfo {
                firsts: vec![Numbered('a', 1), Numbered('b', 2)],
                lasts: vec![Numbered('b', 4)],
                null: false,
                follows: vec![
                    (Numbered('a', 1), vec![Numbered('a', 3), Numbered('b', 4)]),
                    (Numbered('b', 2), vec![Numbered('a', 3), Numbered('b', 4)]),
                    (Numbered('a', 3), vec![Numbered('a', 3), Numbered('b', 4)]),
                    (Numbered('b', 4), vec![])
                ]
            },
            info
        );
    }
}
