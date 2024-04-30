use super::Automata;
use std::{
    fmt::{Display, Formatter, Result},
    hash::Hash,
};

const NB_ATTR: usize = 3;

impl<T, V> Display for Automata<T, V>
where
    T: Eq + Hash + Display + Clone,
    V: Eq + Hash + Display + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "digraph {{\n")?;
        let mut attr: Vec<&str> = Vec::with_capacity(NB_ATTR);
        for (ind, s) in self.states.iter().enumerate() {
            attr.clear();
            if self.finals.contains(&ind) {
                attr.push("peripheries=2");
            }
            if self.initials.contains(&ind) {
                attr.push("shape=diamond");
            }
            write!(
                f,
                "\t{} [label = \"{}\" {}]\n",
                s.value,
                ind,
                attr.join(" ")
            )?;
        }
        let k = self.kosaraju();
        for (ind, sub) in k.iter().enumerate() {
            write!(f, "\tsubgraph cluster{} {{\n", ind)?;
            for s in sub {
                let pos = self
                    .states
                    .iter()
                    .position(|state| s.eq(&state.value))
                    .unwrap();
                write!(f, "\t\t{}\n", pos)?;
            }
            write!(f, "\t}}\n")?;
        }
        for (ind, s) in self.states.iter().enumerate() {
            for key in s.next.keys() {
                for v in s.next.get(key).unwrap() {
                    attr.clear();
                    let pf = k.iter().position(|vec| vec.contains(&s.value));
                    let pt = k
                        .iter()
                        .position(|vec| vec.contains(&self.states[*v].value));
                    if pf != pt {
                        attr.push("color=\"red;0.5:blue\"");
                    }
                    write!(
                        f,
                        "\t{} -> {} [label = \"{}\" {}]\n",
                        ind,
                        v,
                        key,
                        attr.join(" ")
                    )?;
                }
            }
        }
        write!(f, "}}\n")
    }
}
