//! Module containing the information and implementation required for the
//! display of a automata

use crate::automata::in_out::DoorType;

use super::Automata;
use std::{
    fmt::{Display, Formatter, Result, Write},
    hash::Hash,
};

const NB_ATTR: usize = 3;

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Display + Clone,
    V: Eq + Hash + Display + Clone,
{
    /// Represents dot automaton with colors inverted if "inverse" is true
    pub fn fmt_arg(&self, f: &mut dyn Write, inverse: bool) -> Result {
        write!(f, "digraph {{\n\trankdir=LR\n\tbgcolor=transparent\n\tnode [fontname=Cantarell];\n\tedge [fontname=Cantarell];\n")?;
        if inverse {
            write!(
                f,
                "\tcolor=white\n\tnode [color=white, fontcolor=white];\n\tedge [color=white, fontcolor=white];"
            )?;
        }
        let mut attr: Vec<&str> = Vec::with_capacity(NB_ATTR);
        let stype = self.get_states_type();
        for (ind, s) in self.states.iter().enumerate() {
            attr.clear();
            if self.finals.contains(&ind) {
                attr.push("peripheries=2");
            }
            if self.initials.contains(&ind) {
                attr.push("shape=diamond");
            }
            match stype.get(&s.0).unwrap() {
                &DoorType::Both => attr.push("color=purple"),
                &DoorType::In => attr.push("color=red"),
                &DoorType::Out => attr.push("color=blue"),
                &DoorType::None => {}
            }
            write!(f, "\t{} [label = \"{}\" {}]\n", ind, s.0, attr.join(" "))?;
        }
        let k = self.kosaraju();
        for (ind, sub) in k.iter().enumerate() {
            write!(f, "\tsubgraph cluster{} {{\n", ind)?;
            for s in sub {
                let pos = unsafe { self.get_ind_state(s) };
                write!(f, "\t\t{}\n", pos)?;
            }
            write!(f, "\t}}\n")?;
        }
        for ind in 0..self.get_nb_states() {
            for (key, set) in self.follow[ind].iter() {
                for v in set {
                    write!(f, "\t{} -> {} [label = \"{}\"]\n", ind, v, key)?;
                }
            }
        }
        write!(f, "}}\n")
    }

    /// Returns graphical representation of dot graph with colors inverted if
    /// "inverse" is true
    pub fn to_dot(&self, inverse: bool) -> String {
        let mut buf = String::new();
        self.fmt_arg(&mut buf, inverse)
            .expect("a Display implementation returned an error unexpectedly");
        buf
    }
}

impl<T, V> Display for Automata<T, V>
where
    T: Eq + Hash + Display + Clone,
    V: Eq + Hash + Display + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt_arg(f, false)
    }
}
