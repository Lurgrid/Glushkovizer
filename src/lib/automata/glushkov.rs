//! Sous module permettant la gestion d'automate de Glushkov, avec une
//! convertion de RegExp en automate de Glushkov

use std::fmt::Display;

use crate::automata::Automata;
use crate::regexp::RegExp;

impl<T> From<RegExp<T>> for Automata<T, usize>
where
    T: Eq + Ord + Clone + Copy + Display,
{
    fn from(reg: RegExp<T>) -> Self {
        let (a, end) = reg.linearization(1);
        let info = a.get_info();
        let mut g = Automata::new();
        for i in 0..end {
            g.add_state(i);
        }
        for i in 1..end {
            if let Some((_, l)) = info.follows.iter().find(|&(s, _)| s.1 == i) {
                for next in l {
                    let _ = g.add_transition(i, next.1, next.0);
                }
            }
        }
        if info.null {
            let _ = g.add_final(0);
        }
        for f in info.lasts {
            let _ = g.add_final(f.1);
        }
        for i in info.firsts {
            let _ = g.add_transition(0, i.1, i.0);
        }
        let _ = g.add_initial(0);
        g
    }
}

#[cfg(test)]
mod test {
    use crate::automata::Automata;
    use crate::regexp::RegExp;

    #[test]
    fn auto_regexp() {
        let r = RegExp::try_from("a.a");
        assert!(r.is_ok());
        let r = r.unwrap();
        let g = Automata::from(r);
        assert_eq!(g.get_nb_states(), 3);
        assert!(g.accept(&['a', 'a']));
    }
}
