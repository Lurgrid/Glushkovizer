//! Module for kosaraju implementation and strongly connected component
//! extraction

use super::{dfs::DFSInfo, door::DoorType, state::RefState, InnerAutomata};
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

    /// Returns the automata representing the strongly connected components
    /// of the automaton
    pub fn extract_scc(&self) -> Vec<InnerAutomata<'a, T, V>> {
        self.get_door()
            .into_iter()
            .fold(Vec::default(), move |mut acc, l| {
                acc.push(Self {
                    inputs: l
                        .iter()
                        .filter_map(|(rs, dt)| match dt {
                            DoorType::In | DoorType::Both => Some(rs.clone()),
                            _ => None,
                        })
                        .collect(),
                    outputs: l
                        .iter()
                        .filter_map(|(rs, dt)| match dt {
                            DoorType::Out | DoorType::Both => Some(rs.clone()),
                            _ => None,
                        })
                        .collect(),
                    states: l.into_iter().map(|(rs, _)| rs).collect(),
                });
                acc
            })
    }
}
