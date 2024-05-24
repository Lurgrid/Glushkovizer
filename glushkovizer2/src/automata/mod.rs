//! Module allowing automaton management. With the possibility of "created by
//! hand", checks if a word is recognized by this automata. Finally it can also
//! be converted into dot format

pub mod error;
pub mod inner_automata;

use std::hash::Hash;

use inner_automata::InnerAutomata;

#[derive(Clone, Default, Debug)]
pub struct Automata<T: Eq + Hash, V>(InnerAutomata<T, V>);
#[derive(Clone, Default, Debug)]
pub struct SubAutomata<T: Eq + Hash, V>(InnerAutomata<T, V>);
