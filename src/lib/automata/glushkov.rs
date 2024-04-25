//! Sous module permettant la gestion d'automate de Glushkov, avec une
//! convertion de RegExp en automate de Glushkov

use crate::automata::{Automata, FinitAutomata};
use crate::regexp::RegExp;
use std::fmt::Display;
use std::hash::Hash;

impl<T> From<RegExp<T>> for FinitAutomata<T>
where
    T: Clone + Copy + PartialEq + Eq + Hash + Display,
{
    fn from(reg: RegExp<T>) -> Self {
        let (_, end, info) = reg.linearization(1);
        let mut g = FinitAutomata::new();
        for _ in 0..end {
            g.add_state();
        }
        for i in 1..end {
            if let Some(&(_, ref l)) = info.follows.iter().find(|&&(ref s, _)| s.1 == i) {
                for next in l {
                    g.add_transition(i, next.1, next.0);
                }
            }
        }
        if info.null {
            g.add_final(0);
        }
        for f in info.lasts {
            g.add_final(f.1);
        }
        for i in info.firsts {
            g.add_transition(0, i.1, i.0);
        }
        g.add_initial(0);
        g
    }
}

#[cfg(test)]
mod test {
    use crate::automata::{Automata, FinitAutomata};
    use crate::regexp::RegExp;

    #[test]
    fn auto_regexp() {
        let r = RegExp::try_from("a.a");
        assert!(r.is_ok());
        let r = r.unwrap();
        let g = FinitAutomata::from(r);
        assert_eq!(g.get_nb_states(), 3);
        assert!(g.accept(&['a', 'a']));
    }
}
