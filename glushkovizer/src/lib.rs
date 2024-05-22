#![warn(missing_docs)]
//! Module for managing regular expressions, automata and converting regular
//! expressions into Glushkov automata.
//!
//! # Example
//!
//! Here's an example of how it could be used:
//! ```rust
//! use glushkovizer::automata::Automata;
//! use glushkovizer::regexp::RegExp;
//!
//! fn main() {
//!     let a = RegExp::try_from("a*.b*");
//!     if let Err(s) = a {
//!         eprintln!("Error ! {}", s);
//!         return;
//!     }
//!     let a = a.unwrap();
//!     let g = Automata::from(a);
//!     println!("{}", g);
//!     println!(
//!         "L'automate reconnais le mot ?: {}",
//!         g.accept("ab".chars().collect::<Vec<char>>().iter())
//!     );
//! }
//! ```
pub mod automata;
pub mod regexp;
