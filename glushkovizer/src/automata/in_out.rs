//! Module containing the information and implementation required for
//! retrieving the state type of a strongly connected component

use std::{collections::HashMap, hash::Hash, ops::AddAssign};

use super::Automata;

#[derive(PartialEq, Clone)]
/// Type that a state can be
pub enum DoorType {
    /// Represents the fact that the state is an input door
    In,
    /// Represents the fact that the state is an output doorr
    Out,
    /// Represents the fact that the state is both an input door and an
    /// output door
    Both,
    /// Represents the fact that the state is neither an input nor an output
    /// gate
    None,
}

impl AddAssign<DoorType> for DoorType {
    fn add_assign(&mut self, rhs: DoorType) {
        *self = match self {
            &mut Self::In if rhs == Self::Out => Self::Both,
            &mut Self::Out if rhs == Self::In => Self::Both,
            &mut Self::None => rhs,
            _ => self.clone(),
        }
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Returns an map between a state and its type
    pub fn get_states_type(&self) -> HashMap<V, DoorType> {
        self.get_states_type_ind()
            .into_iter()
            .map(|(k, v)| (self.states[k].0.clone(), v))
            .collect()
    }

    pub(crate) fn get_states_type_ind(&self) -> HashMap<usize, DoorType> {
        let k = self.kosaraju_ind();
        let mut stype = HashMap::new();
        (0..self.get_nb_states()).for_each(|_| {
            stype.insert(0, DoorType::None);
        });
        (0..self.get_nb_states()).for_each(|ind| {
            if self.initials.contains(&ind) {
                *stype.get_mut(&ind).unwrap() += DoorType::In;
            }
            if self.finals.contains(&ind) {
                *stype.get_mut(&ind).unwrap() += DoorType::Out;
            }
            self.follow[ind].values().for_each(|set| {
                for v in set {
                    let pf = k.iter().position(|vec| vec.contains(&ind));
                    let pt = k.iter().position(|vec| vec.contains(v));
                    if pf != pt {
                        *stype.get_mut(&ind).unwrap() += DoorType::Out;
                        *stype.get_mut(v).unwrap() += DoorType::In;
                    }
                }
            });
        });
        stype
    }
}
