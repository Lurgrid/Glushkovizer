//! Module containing all the information and implementation required for the
//! deep path

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use super::{state::RefState, InnerAutomata};

#[derive(Debug)]
/// Structure grouping together the information of a depth path of the
/// the automaton
pub struct DFSInfo<V> {
    /// State set deﬁned by prefix\[i\] contains the state that is being
    /// discovered at time i
    pub prefix: Vec<V>,
    /// Set of states deﬁned by suffix\[i\] contains the state we've finished
    /// to explore at time i
    pub suffix: Vec<V>,
    /// Set, deﬁned by predecessor\[u\] = None, if u is a root of the
    /// exploration forest and by predecessor\[u\] is the predecessor of the
    /// vertex u otherwise
    pub predecessor: HashMap<V, V>,
}

impl<'a, T, V> InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns the depth first search information. This route is done with the
    /// follows if "backward" is ```false``` and otherwise with the previous
    pub fn dfs(
        &self,
        order: Vec<RefState<'a, T, V>>,
        backward: bool,
    ) -> DFSInfo<RefState<'a, T, V>> {
        let mut info = DFSInfo::<RefState<'a, T, V>> {
            prefix: Vec::with_capacity(self.states_count()),
            suffix: Vec::with_capacity(self.states_count()),
            predecessor: HashMap::default(),
        };
        let mut color: HashMap<&RefState<'a, T, V>, bool> =
            order.iter().map(|rs| (rs, true)).collect();
        for rs in order.iter() {
            if unsafe { *color.get(&rs).unwrap_unchecked() } {
                self.visit_in_depth(&order, rs, &mut color, &mut info, backward)
            }
        }
        info
    }

    fn visit_in_depth(
        &self,
        order: &Vec<RefState<'a, T, V>>,
        rs: &RefState<'a, T, V>,
        color: &mut HashMap<&RefState<'a, T, V>, bool>,
        info: &mut DFSInfo<RefState<'a, T, V>>,
        backward: bool,
    ) {
        unsafe { *color.get_mut(rs).unwrap_unchecked() = false };
        info.prefix.push(rs.clone());
        let mut p: HashSet<&RefState<'a, T, V>> = HashSet::new();
        let rmut = rs.as_ref().borrow();
        if backward {
            rmut.get_previous().for_each(|(_, set)| {
                p.extend(set);
            });
        } else {
            rmut.get_follows().for_each(|(_, set)| {
                p.extend(set);
            });
        }
        let mut p: Vec<&RefState<'a, T, V>> = p.into_iter().collect();
        p.sort_by_key(|&ind| order.iter().position(|x| x == ind));
        p.into_iter().for_each(|rs2| {
            if unsafe { *color.get(&rs2).unwrap_unchecked() } {
                info.predecessor.insert(rs2.clone(), rs.clone());
                self.visit_in_depth(order, rs2, color, info, backward)
            }
        });
        info.suffix.push(rs.clone())
    }
}
