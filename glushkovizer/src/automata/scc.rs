//! Module permantant l'implementation de kosaraju

use crate::automata::dfs::DFSInfo;

use super::{in_out::DoorType, Automata};
use std::{collections::HashSet, hash::Hash};

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Apply the kosaraju algorithm to the automaton and return the strongly
    /// connected components.
    pub fn kosaraju(&self) -> Vec<HashSet<V>> {
        self.kosaraju_ind()
            .into_iter()
            .map(|set| {
                set.into_iter()
                    .map(|ind| self.states[ind].0.clone())
                    .collect()
            })
            .collect()
    }

    /// Apply the kosaraju algorithm to the automaton and return the strongly
    /// connected components.
    pub(crate) fn kosaraju_ind(&self) -> Vec<HashSet<usize>> {
        let DFSInfo {
            prefix: _,
            suffix: mut order,
            predecessor: _,
        } = unsafe { self.get_dfs_unchecked((0..self.get_nb_states()).collect()) };
        let inverse = self.get_inverse();
        order.reverse();
        let DFSInfo {
            prefix,
            suffix: _,
            predecessor: res,
        } = unsafe { inverse.get_dfs_unchecked(order) };
        let mut r = Vec::new();
        let mut cur = HashSet::new();
        for pos in prefix {
            if res[pos].is_none() {
                if cur.len() != 0 {
                    r.push(cur);
                }
                cur = HashSet::from([pos]);
            } else {
                cur.insert(pos);
            }
        }
        r.push(cur);
        r
    }

    /// Returns the sub-automata of the strongly connected components, where the
    /// initial states are the input gates and the final states are the output
    /// gates
    pub fn extract_scc(&self) -> Vec<Self> {
        let mut res = Vec::new();
        let stype = self.get_states_type();
        for scc in self.kosaraju() {
            let mut g = self.get_subautomata(&scc).unwrap();
            for s in scc {
                match stype.get(&s).unwrap() {
                    &DoorType::Both => {
                        g.add_initial(s.clone()).unwrap();
                        g.add_final(s).unwrap();
                    }
                    &DoorType::In => {
                        g.add_initial(s).unwrap();
                    }
                    &DoorType::Out => {
                        g.add_final(s).unwrap();
                    }
                    _ => {}
                }
            }
            res.push(g);
        }
        res
    }
}
