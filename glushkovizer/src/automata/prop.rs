//! Module for testing the properties of an automaton

use super::in_out::DoorType;
use super::Automata;
use std::collections::HashSet;
use std::hash::Hash;

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns if the automaton is standard
    pub fn is_standard(&self) -> bool {
        self.initials.len() == 1
            && self.follow.iter().all(|map| {
                map.values().all(|set| {
                    !set.contains(unsafe { self.initials.iter().next().unwrap_unchecked() })
                })
            })
    }

    /// Returns if the automaton is deterministic
    pub fn is_deterministic(&self) -> bool {
        self.is_standard()
            && self
                .follow
                .iter()
                .all(|map| map.values().all(|nexts| nexts.len() <= 1))
    }

    /// Returns if the automaton is fully deterministic
    pub fn is_fully_deterministic(&self) -> bool {
        self.is_standard()
            && self
                .follow
                .iter()
                .all(|map| map.values().all(|nexts| nexts.len() == 1))
    }

    /// Returns if the automaton is homogeneous
    pub fn is_homogeneous(&self) -> bool {
        let mut dir: Vec<Option<T>> = vec![None; self.states.len()];

        self.follow.iter().all(|f| {
            f.iter().all(|(key, set)| {
                set.iter().all(|to| {
                    match &dir[*to] {
                        None => dir[*to] = Some(key.clone()),
                        Some(val) if val.eq(key) => (),
                        _ => return false,
                    };
                    return true;
                })
            })
        })
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Returns if the automaton is accessible
    pub fn is_accessible(&self) -> bool {
        let mut p: Vec<usize> = (0..self.get_nb_states()).collect();
        p.sort_by_key(|ind| {
            self.initials
                .iter()
                .position(|x| x == ind)
                .unwrap_or(self.get_nb_states())
        });

        unsafe { self.get_dfs_unchecked(p) }
            .predecessor
            .into_iter()
            .fold(0, |acc, opt| if opt.is_none() { acc + 1 } else { acc })
            <= 1
    }

    /// Returns if the automaton is coaccessible
    pub fn is_coaccessible(&self) -> bool {
        self.get_inverse().is_accessible()
    }

    /// Returns whether the automaton is strongly connected
    pub fn is_strongly_connected(&self) -> bool {
        self.kosaraju().len() <= 1
    }

    /// Returns whether the automaton is a orbit
    pub fn is_orbit(&self) -> bool {
        self.is_strongly_connected()
            && (self.get_nb_states() != 1 || self.follow[0].values().any(|set| set.len() > 0))
    }

    /// Returns whether the orbit is stable
    pub fn is_stable(&self) -> bool {
        if !self.is_orbit() {
            return false;
        }
        let mut inp = HashSet::new();
        let mut out = HashSet::new();
        self.get_states_type_ind()
            .into_iter()
            .for_each(|(i, t)| match t {
                DoorType::In => {
                    inp.insert(i);
                }
                DoorType::Out => {
                    out.insert(i);
                }
                DoorType::Both => {
                    inp.insert(i);
                    out.insert(i);
                }
                _ => {}
            });

        inp.iter().all(|i| {
            self.follow[*i]
                .values()
                .any(|set| set.intersection(&out).count() != 0)
        }) && out.into_iter().all(|i| {
            self.follow[i]
                .values()
                .any(|set| set.intersection(&inp).count() != 0)
        })
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
}
