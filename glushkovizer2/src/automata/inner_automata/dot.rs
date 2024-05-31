//! Module containing the information and implementation required for the point
//! display of a automata.

use crate::automata::DoorType;

use super::InnerAutomata;
use std::{
    fmt::{Display, Write},
    hash::Hash,
};

const NB_ATTR: usize = 3;

impl<'a, T, V> InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone + Display,
    V: Display,
{
    /// Returns the DOT representation of the automaton
    pub fn to_dot(&self) -> Result<String, std::fmt::Error> {
        let mut f = String::new();
        write!(
            f,
            "digraph {{\n\trankdir=LR\n\tbgcolor=transparent\n\tnode \
            [fontname=Cantarell];\n\tedge [fontname=Cantarell];\n"
        )?;
        let mut attr: Vec<&str> = Vec::with_capacity(NB_ATTR);
        let stype = self.get_door();
        stype.iter().try_for_each(|l| {
            l.iter().try_for_each(|(rs, tdoor)| {
                attr.clear();
                if self.outputs.contains(rs) {
                    attr.push("peripheries=2");
                }
                if self.inputs.contains(rs) {
                    attr.push("shape=diamond");
                }
                match tdoor {
                    DoorType::Both => attr.push("color=purple"),
                    DoorType::In => attr.push("color=red"),
                    DoorType::Out => attr.push("color=blue"),
                    DoorType::None => {}
                }
                write!(
                    f,
                    "\t{} [label = \"{}\" {}]\n",
                    rs.as_ptr() as usize,
                    rs.as_ref().get_value(),
                    attr.join(" ")
                )
            })
        })?;
        stype.iter().enumerate().try_for_each(|(ind, sub)| {
            write!(f, "\tsubgraph cluster{} {{\n", ind)?;
            sub.into_iter()
                .try_for_each(|(s, _)| write!(f, "\t\t{}\n", s.as_ptr() as usize))?;
            write!(f, "\t}}\n")
        })?;
        self.states.iter().try_for_each(|from| {
            from.as_ref().get_follows().try_for_each(|(symbol, set)| {
                set.into_iter().try_for_each(|to| {
                    write!(
                        f,
                        "\t{} -> {} [label = \"{}\"]\n",
                        from.as_ptr() as usize,
                        to.as_ptr() as usize,
                        symbol
                    )
                })
            })
        })?;
        write!(f, "}}\n")?;
        Ok(f)
    }
}
