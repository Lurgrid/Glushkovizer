//! Module for determining the type of a state in its component strongly
//! connected

use super::{InnerAutomata, RefState};
use std::{hash::Hash, ops::AddAssign};

#[derive(PartialEq, Clone, Debug)]
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

impl<'a, T, V> InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns the result of the kosaraju algorithm on the automaton, i.e. the
    /// set of strongly connected components. Where each element of the
    /// strongly connected components is a pair between the reference of the
    ///  state and the type of gate it is.
    pub fn get_door(&self) -> Vec<Vec<(RefState<'a, T, V>, DoorType)>> {
        self.kosaraju()
            .into_iter()
            .map(|ref l| {
                l.into_iter()
                    .map(|rs| {
                        let mut dt = DoorType::None;
                        rs.as_ref().borrow().get_follows().for_each(|(_, set)| {
                            set.iter().for_each(|rs| {
                                if !l.contains(rs) {
                                    dt += DoorType::Out;
                                }
                            });
                        });
                        rs.as_ref().borrow().get_previous().for_each(|(_, set)| {
                            set.iter().for_each(|rs| {
                                if !l.contains(rs) {
                                    dt += DoorType::In;
                                }
                            });
                        });
                        if self.inputs.contains(rs) {
                            dt += DoorType::In;
                        }
                        if self.outputs.contains(rs) {
                            dt += DoorType::Out;
                        }
                        (rs.clone(), dt)
                    })
                    .collect()
            })
            .collect()
    }
}
