//! Module for testing the properties of an automaton

use super::Automata;
use std::{collections::HashSet, hash::Hash};

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone + Eq,
{
    /// Returns if the automaton is standard
    pub fn is_standard(&self) -> bool {
        self.initials.len() <= 1
    }

    /// Returns if the automaton is deterministic
    pub fn is_deterministic(&self) -> bool {
        if !self.is_standard() {
            return false;
        }
        let mut t_set = HashSet::new();
        for ind in 0..self.get_nb_states() {
            t_set.clear();
            for nexts in self.follow[ind].values() {
                for next in nexts.iter() {
                    if !t_set.insert(*next) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Returns if the automaton is homogeneous
    pub fn is_homogeneous(&self) -> bool {
        let mut dir: Vec<Option<T>> = vec![None; self.states.len()];

        for f in self.follow.iter() {
            for (key, set) in f.iter() {
                for to in set.iter() {
                    match &dir[*to] {
                        None => dir[*to] = Some(key.clone()),
                        Some(val) if val.eq(key) => (),
                        _ => return false,
                    }
                }
            }
        }
        true
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Returns whether the automaton is strongly connected
    pub fn is_strongly_connected(&self) -> bool {
        unsafe { self.get_dfs_unchecked((0..self.get_nb_states()).collect()) }
            .predecessor
            .into_iter()
            .fold(0, |acc, opt| if opt.is_none() { acc + 1 } else { acc })
            <= 1
    }

    /// Returns whether the automaton is a maximal orbit
    pub fn is_maximal_orbit(&self) -> bool {
        self.is_strongly_connected()
            && (self.get_nb_states() != 1 || self.follow[0].values().any(|set| set.len() > 0))
    }
}

#[cfg(test)]
mod test {
    use crate::{automata::Automata, regexp::RegExp};

    #[test]
    fn maximal_orbit() {
        let r = RegExp::try_from("(a+b).a*.b*.(a+b)*");
        assert!(r.is_ok());
        let r = r.unwrap();
        let a = Automata::from(r);
        let scc = a.extract_scc();
        assert!(scc[3].is_maximal_orbit());
        assert!(scc[4].is_maximal_orbit());
        assert!(scc[5].is_maximal_orbit());
    }
}
