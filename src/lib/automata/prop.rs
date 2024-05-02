//! Module de tests des propriétées d'un automates

use super::Automata;
use std::{collections::HashSet, hash::Hash};

impl<T, V> Automata<T, V>
where
    T: Eq + Hash,
{
    /// Renvoie vrai si l'automate est standart et faux sinon
    pub fn is_standard(&self) -> bool {
        self.initials.len() <= 1
    }

    /// Renvoie vrai si l'automate est deterministe
    pub fn is_deterministic(&self) -> bool {
        if !self.is_standard() {
            return false;
        }
        let mut t_set = HashSet::new();
        for state in self.states.iter() {
            t_set.clear();
            for nexts in state.next.values() {
                for next in nexts.iter() {
                    if !t_set.insert(*next) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    /// Renvoie vrai si l'automate est homogène
    pub fn is_homogeneous(&self) -> bool {
        self.states.iter().all(|s| s.prev.keys().len() < 2)
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Renvoie vrai si l'automate est une orbite maximal
    pub fn is_maximal_orbit(&self) -> bool {
        if !self.is_homogeneous() || self.kosaraju().len() > 1 {
            return false;
        }
        self.states.iter().all(|s| {
            for set in s.next.values() {
                for next in set {
                    if self.initials.contains(next) {
                        return true;
                    }
                }
            }
            return false;
        })
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
