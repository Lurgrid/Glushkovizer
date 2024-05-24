//! Module for managing regular expressions. You can create a regular expression
//! "by hand" or "parse" it from a string containing the following operations:
//!
//! - ```a```: Where "a" is a letter of the alphabet between 'a' and 'z' or 'A'
//!     and 'Z'
//!
//! - ```$```: Character used to represent epsilon (the empty word)
//!
//! - ```expr*```: Allows infinite repetition, with a repetition of
//!     zero times included
//!
//! - ```expr.expr```: Represents the concatenation of the two regular
//!     expressions
//!
//! - ```expr+expr```: Represents the "or" between the two regular
//!     expressions
//!
//! # Example
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
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
};

lrlex_mod!("regexp/reg.l");
lrpar_mod!("regexp/reg.y");

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
/// Name of an enum whose purpose is to represent a regular expression using a
/// tree, composed of T-type symbols
pub enum RegExp<T> {
    /// Element of the enum to represent the empty word epsilon.
    Epsilon,
    /// Enum element to represent a letter of the Sigma alphabet, where each
    /// element is of type T
    Symbol(T),
    /// Enum element to represent the repetition of an expression. This
    /// repetition is infinite and includes the empty word
    Repeat(Box<RegExp<T>>),
    /// Enum element to represent the concatenation of two regular
    /// sub-expressions
    Concat(Box<RegExp<T>>, Box<RegExp<T>>),
    /// Enum element to represent the union of two regular sub-expressions
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

impl TryFrom<String> for RegExp<char> {
    type Error = String;

    fn try_from(regexp: String) -> Result<RegExp<char>, Self::Error> {
        let lexerdef = reg_l::lexerdef();
        let lexer = lexerdef.lexer(regexp.as_str());
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Structure to represent numbered symbols
pub struct Numbered<T>(pub T, pub usize);

/// Structure containing all the information of a regular expression
pub struct FLNF<T> {
    /// Set of firsts of a regular expression
    pub firsts: HashSet<T>,
    /// Set of the last of a regular expression
    pub lasts: HashSet<T>,
    /// Is the empty word recognized
    pub null: bool,
    /// An map representing follows
    pub follows: HashMap<T, HashSet<T>>,
}

impl<T> RegExp<T>
where
    T: Eq + Hash + Clone,
{
    /// Creates from a regular expression another regular expression where each
    /// symbol will be numbered starting from "start" and returns it in a pair,
    /// accompanied by "start" + the number of numbered symbols
    pub fn linearization_start(&self, start: usize) -> (RegExp<Numbered<T>>, usize) {
        match self {
            RegExp::Epsilon => (RegExp::Epsilon, start),
            RegExp::Symbol(v) => {
                let s = Numbered(v.clone(), start);
                let r = RegExp::Symbol(s);
                let end = start + 1;
                (r, end)
            }
            RegExp::Repeat(c) => {
                let (nc, end) = c.linearization_start(start);
                (RegExp::Repeat(Box::new(nc)), end)
            }
            RegExp::Or(l, r) => {
                let (nl, end) = l.linearization_start(start);
                let (nr, end) = r.linearization_start(end);
                (RegExp::Or(Box::new(nl), Box::new(nr)), end)
            }
            RegExp::Concat(l, r) => {
                let (nl, end) = l.linearization_start(start);
                let (nr, end) = r.linearization_start(end);
                (RegExp::Concat(Box::new(nl), Box::new(nr)), end)
            }
        }
    }

    /// Creates from a regular expression another regular expression where each
    /// symbol will be numbered starting from 1 and returns it
    pub fn linearization(&self) -> RegExp<Numbered<T>> {
        self.linearization_start(1).0
    }

    /// Returns regular expression information.
    pub fn get_flnf(&self) -> FLNF<T> {
        match self {
            RegExp::Epsilon => FLNF {
                firsts: Default::default(),
                lasts: Default::default(),
                null: true,
                follows: Default::default(),
            },
            RegExp::Symbol(v) => FLNF {
                firsts: HashSet::from([v.clone()]),
                lasts: HashSet::from([v.clone()]),
                null: false,
                follows: HashMap::from([(v.clone(), HashSet::new())]),
            },
            RegExp::Repeat(c) => {
                let mut gi = c.get_flnf();
                gi.null = true;
                for last in gi.lasts.iter() {
                    if let Some(f) = gi.follows.get_mut(last) {
                        gi.firsts.iter().for_each(|d| {
                            f.insert(d.clone());
                        });
                    }
                }
                gi
            }
            RegExp::Or(l, r) => {
                let mut gil = l.get_flnf();
                let gir = r.get_flnf();
                gir.firsts.into_iter().for_each(|d| {
                    gil.firsts.insert(d);
                });
                gir.lasts.into_iter().for_each(|d| {
                    gil.lasts.insert(d);
                });
                gil.null = gil.null || gir.null;
                for (k, v) in gir.follows {
                    match gil.follows.get_mut(&k) {
                        None => {
                            gil.follows.insert(k, v);
                        }
                        Some(l) => {
                            v.into_iter().for_each(|d| {
                                l.insert(d);
                            });
                        }
                    }
                }
                gil
            }
            RegExp::Concat(l, r) => {
                let mut gil = l.get_flnf();
                let gir = r.get_flnf();
                for last in gil.lasts.iter() {
                    if let Some(f) = gil.follows.get_mut(last) {
                        gir.firsts.iter().for_each(|d| {
                            f.insert(d.clone());
                        });
                    }
                }
                for (k, v) in gir.follows {
                    match gil.follows.get_mut(&k) {
                        None => {
                            gil.follows.insert(k, v);
                        }
                        Some(l) => {
                            v.into_iter().for_each(|d| {
                                l.insert(d);
                            });
                        }
                    }
                }
                if gil.null {
                    gir.firsts.into_iter().for_each(|d| {
                        gil.firsts.insert(d);
                    });
                }
                if gir.null {
                    gir.lasts.into_iter().for_each(|d| {
                        gil.lasts.insert(d);
                    });
                } else {
                    gil.lasts = gir.lasts;
                }
                gil.null = gil.null && gir.null;
                gil
            }
        }
    }
}

impl<T: Display> Display for RegExp<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Epsilon => write!(f, "$"),
            Self::Symbol(s) => write!(f, "{}", s),
            Self::Repeat(r) => write!(f, "{}*", r),
            Self::Or(r, l) => write!(f, "({}+{})", r, l),
            Self::Concat(r, l) => write!(f, "({}.{})", r, l),
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
            "Ok(Or(Concat(Concat(Repeat(Or(Symbol('a'), Symbol('b'))), \
            Symbol('a')), Symbol('b')), Epsilon))",
            format!("{:?}", a)
        );
    }

    #[test]
    fn regex2() {
        let a = RegExp::try_from("(a + b) . ( a* . b)");
        assert_eq!(
            "Ok(Concat(Or(Symbol('a'), Symbol('b')), \
            Concat(Repeat(Symbol('a')), Symbol('b'))))",
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
        let (a, n) = a.unwrap().linearization_start(1);
        let _ = a.get_flnf();
        assert_eq!(
            "Concat(Or(Symbol(Numbered('a', 1)), Symbol(Numbered('b', 2))), \
            Concat(Repeat(Symbol(Numbered('a', 3))), Symbol(Numbered('b', 4))))",
            format!("{:?}", a)
        );
        assert_eq!(5, n);
    }
}
