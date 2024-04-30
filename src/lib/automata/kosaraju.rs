//! Module permantant l'implementation de kosaraju

use crate::automata::dfs::DFSInfo;

use super::Automata;
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
            .get_dfs(self.states.iter().map(|s| s.value.clone()).collect())
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
            let pos = inverse.states.iter().position(|s| s.value == p).unwrap();
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
}
