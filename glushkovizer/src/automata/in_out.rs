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
        let k = self.kosaraju();
        let mut stype = HashMap::new();
        self.states.iter().for_each(|s| {
            stype.insert(s.0.clone(), DoorType::None);
        });
        for (ind, s) in self.states.iter().enumerate() {
            if self.initials.contains(&ind) {
                *stype.get_mut(&s.0).unwrap() += DoorType::In;
            }
            if self.finals.contains(&ind) {
                *stype.get_mut(&s.0).unwrap() += DoorType::Out;
            }
            for set in self.follow[ind].values() {
                for v in set {
                    let pf = k.iter().position(|vec| vec.contains(&s.0));
                    let pt = k.iter().position(|vec| vec.contains(&self.states[*v].0));
                    if pf != pt {
                        *stype.get_mut(&s.0).unwrap() += DoorType::Out;
                        *stype.get_mut(&self.states[*v].0).unwrap() += DoorType::In;
                    }
                }
            }
        }
        stype
    }
}
