#![warn(missing_docs)]
//! Module ayant pour but la gestion d'expression régulière, d'automate et la
//! convertion d'expression régulière en automate de Glushkov.
//!
//! # Exemple
//!
//! Voici un exemple qui illustre bien l'utilisation qu'on pourrait en faire:
//! ```rust
//! use glushkovizer::automata::Automata;
//! use glushkovizer::regexp::RegExp;
//! use petgraph::dot::Dot;
//!
//! fn main() {
//!     let a = RegExp::try_from("a*.b*");
//!     if let Err(s) = a {
//!         eprintln!("Error ! {}", s);
//!         return;
//!     }
//!     let a = a.unwrap();
//!     let g = Automata::from(a);
//!     println!("{}", Dot::with_config(&g.get_graph(), &[]));
//!     println!(
//!         "L'automate reconnais le mot ?: {}",
//!         g.accept(&("ab".chars().collect::<Vec<char>>()[..]))
//!     );
//! }
//! ```
pub mod automata;
pub mod regexp;
