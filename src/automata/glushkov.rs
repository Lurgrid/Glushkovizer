//! Module for Glushkov automaton management, with a conversion of [RegExp]
//! into a Glushkov automaton

use super::{AddStates, Automata, MutTransition};
use crate::regexp::RegExp;
use std::hash::Hash;

impl<'a, T> From<RegExp<T>> for Automata<'a, T, usize>
where
    T: Eq + Hash + Clone,
{
    fn from(reg: RegExp<T>) -> Self {
        let (a, end) = reg.linearization_start(1);
        let info = a.get_flnf();
        let g = Automata::new();
        for i in 0..end {
            g.add_state(i);
        }
        unsafe {
            for i in 1..end {
                if let Some((_, l)) = info.follows.iter().find(|&(s, _)| s.1 == i) {
                    for next in l {
                        g.add_transition(&i, &next.1, next.0.clone())
                            .unwrap_unchecked();
                    }
                }
            }
            if info.null {
                g.add_final(&0).unwrap_unchecked();
            }
            for f in info.lasts {
                g.add_final(&f.1).unwrap_unchecked();
            }
            for i in info.firsts {
                let _ = g.add_transition(&0, &i.1, i.0);
            }
            let _ = g.add_initial(&0);
        }
        g
    }
}
