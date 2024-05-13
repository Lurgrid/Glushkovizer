//! Module permantant l'implementation de kosaraju

use crate::automata::dfs::DFSInfo;

use super::{in_out::DoorType, Automata};
use std::hash::Hash;

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Appliquer l'algorithme de kosaraju sur l'autome et renvoie les
    /// composantes fortement connexe.
    pub fn kosaraju(&self) -> Vec<Vec<V>> {
        let DFSInfo {
            prefix: _,
            suffix: mut order,
            predecessor: _,
        } = self
            .get_dfs(self.states.iter().map(|s| s.0.clone()).collect())
            .unwrap();
        let inverse = self.get_inverse();
        order.reverse();
        let DFSInfo {
            prefix,
            suffix: _,
            predecessor: res,
        } = inverse.get_dfs(order).unwrap();
        let mut r = Vec::new();
        let mut cur = Vec::new();
        for p in prefix {
            let pos = unsafe { inverse.get_ind_state(&p) };
            if res[pos].is_none() {
                if cur.len() != 0 {
                    r.push(cur);
                }
                cur = vec![p];
            } else {
                cur.push(p)
            }
        }
        r.push(cur);
        r
    }

    /// Renvoie les sous automate des composantes fortement connexe, où les
    /// initiaux sont les portes entrante et les finaux les portes sortante
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
