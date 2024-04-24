//! Sous module permettant la gestion d'expression régulière
//! # Exemple
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
//!     let (b, n) = a.numbered(1);
//! }
//! ```

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use std::fmt::Debug;

lrlex_mod!("reg.l");
lrpar_mod!("reg.y");

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
pub struct Numbered<T>(pub T, pub usize);

impl<T: Clone + Copy> RegExp<T> {
    /// Crée à partir d'une expression régulière une autre expression
    ///     régulière où chaque symbole sera numéroté en partant de "start" et
    ///     renvoie celle-ci dans un couple, accompagné de "start" + le nombre
    ///     de symbole numéroté.
    pub fn numbered(&self, start: usize) -> (RegExp<Numbered<T>>, usize) {
        match self {
            RegExp::Concat(l, r) => {
                let (nl, end) = l.numbered(start);
                let (nr, end) = r.numbered(end);
                (RegExp::Concat(Box::new(nl), Box::new(nr)), end)
            }
            RegExp::Or(l, r) => {
                let (nl, end) = l.numbered(start);
                let (nr, end) = r.numbered(end);
                (RegExp::Or(Box::new(nl), Box::new(nr)), end)
            }
            RegExp::Epsilon => (RegExp::Epsilon, start),
            RegExp::Symbol(v) => {
                let r = RegExp::Symbol(Numbered(*v, start));
                let end = start + 1;
                (r, end)
            }
            RegExp::Times(c) => {
                let (nc, end) = c.numbered(start);
                (RegExp::Times(Box::new(nc)), end)
            }
        }
    }
}

#[cfg(test)]
mod test {
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
        let a = a.unwrap().numbered(1).0;
        assert_eq!(
            "Concat(Or(Symbol(Numbered('a', 1)), Symbol(Numbered('b', 2))), \
            Concat(Times(Symbol(Numbered('a', 3))), Symbol(Numbered('b', 4))))",
            format!("{:?}", a)
        );
    }
}
