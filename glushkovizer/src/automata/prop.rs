//! Module de tests des propriétées d'un automates

use super::{in_out::DoorType, Automata};
use std::{collections::HashSet, hash::Hash};

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone + Eq,
{
    /// Renvoie si l'automate est standart
    pub fn is_standard(&self) -> bool {
        self.initials.len() <= 1
    }

    /// Renvoie si l'automate est deterministe
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
        return true;
    }

    /// Renvoie si l'automate est homogène
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
        return true;
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Renvoie si l'automate est une orbite
    pub fn is_orbit(&self) -> bool {
        if !self.is_homogeneous() || self.kosaraju().len() > 1 {
            return false;
        }
        (0..self.get_nb_states()).all(|i| {
            for set in self.follow[i].values() {
                for next in set {
                    if self.initials.contains(next) {
                        return true;
                    }
                }
            }
            return false;
        })
    }

    /// Renvoie si l'automate est une orbite maximal
    pub fn is_maximal_orbit(&self) -> bool {
        if !self.is_homogeneous() {
            return false;
        }
        let scc = self.extract_scc();
        if scc.len() != 1 {
            return false;
        }
        self.get_states_type()
            .into_iter()
            .filter_map(|(k, v)| match v {
                DoorType::Out => Some(unsafe { self.get_ind_state(&k) }),
                _ => None,
            })
            .all(|ind| self.follow[ind].is_empty())
    }
}

#[cfg(test)]
mod test {
    use crate::{automata::Automata, regexp::RegExp};

    #[test]
    fn orbit() {
        let r = RegExp::try_from("(a+b).a*.b*.(a+b)*");
        assert!(r.is_ok());
        let r = r.unwrap();
        let a = Automata::from(r);
        let scc = a.extract_scc();
        assert!(scc[3].is_orbit());
        assert!(scc[4].is_orbit());
        assert!(scc[5].is_orbit());
    }

    #[test]
    fn maximal_orbit() {
        let r = RegExp::try_from("(a+b).a*.b*.(a+b)*");
        assert!(r.is_ok());
        let r = r.unwrap();
        let a = Automata::from(r);
        let scc = a.extract_scc();
        assert!(scc[3].is_orbit());
        assert!(scc[4].is_orbit());
        assert!(scc[5].is_orbit());
    }
}
