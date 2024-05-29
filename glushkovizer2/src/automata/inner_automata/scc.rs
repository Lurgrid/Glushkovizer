//! Module for kosaraju implementation and strongly connected component
//! extraction

use super::{dfs::DFSInfo, state::RefState, InnerAutomata};
use std::hash::Hash;

impl<'a, T, V> InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns the result of the kosaraju algorithm on the graph, the set of
    /// strongly connected components.
    pub fn kosaraju(&self) -> Vec<Vec<RefState<'a, T, V>>> {
        let DFSInfo {
            prefix: _,
            suffix: mut order,
            predecessor: _,
        } = self.dfs(self.states().cloned().collect(), false);
        order.reverse();
        let DFSInfo {
            prefix,
            suffix: _,
            predecessor: res,
        } = self.dfs(order, true);
        let mut r = Vec::new();
        let mut cur = Vec::new();
        for pos in prefix {
            if res.get(&pos).is_none() {
                if cur.len() != 0 {
                    r.push(cur);
                }
                cur = vec![pos];
            } else {
                cur.push(pos);
            }
        }
        r.push(cur);
        r
    }
}
