//! Module for testing the properties of an automaton

use super::{dfs::DFSInfo, door::DoorType, InnerAutomata};
use std::collections::HashSet;
use std::hash::Hash;

impl<'a, T, V> InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns if the automaton is standard
    pub fn is_standard(&self) -> bool {
        if let Some(rs) = self.inputs().next() {
            return rs.as_ref().get_previous().count() == 0;
        }
        return false;
    }

    /// Returns if the automaton is deterministic
    pub fn is_deterministic(&self) -> bool {
        self.is_standard() && self.states().all(|rs| rs.get_symbols_out_count() <= 1)
    }

    /// Returns if the automaton is fully deterministic
    pub fn is_fully_deterministic(&self) -> bool {
        self.is_standard() && self.states().all(|rs| rs.get_symbols_out_count() == 1)
    }

    /// Returns if the automaton is homogeneous
    pub fn is_homogeneous(&self) -> bool {
        self.states().all(|rs| rs.get_symbols_in_count() <= 1)
    }

    /// Returns if the automaton is accessible
    pub fn is_accessible(&self) -> bool {
        let mut order = Vec::default();
        self.inputs().for_each(|rs| order.push(rs.clone()));
        self.states().for_each(|rs| {
            if !order.contains(rs) {
                order.push(rs.clone())
            }
        });
        let DFSInfo {
            prefix: _,
            suffix: _,
            predecessor,
        } = self.dfs(order, false);
        let mut res = false;
        self.states()
            .try_for_each(|rs| {
                if predecessor.get(rs).is_none() {
                    if res {
                        return Err(());
                    } else {
                        res = true;
                    }
                }
                Ok(())
            })
            .is_ok()
    }

    /// Returns if the automaton is coaccessible
    pub fn is_coaccessible(&self) -> bool {
        let mut order = Vec::default();
        self.outputs().for_each(|rs| order.push(rs.clone()));
        self.states().for_each(|rs| {
            if !order.contains(rs) {
                order.push(rs.clone())
            }
        });
        let DFSInfo {
            prefix: _,
            suffix: _,
            predecessor,
        } = self.dfs(order, true);
        let mut res = false;
        self.states()
            .try_for_each(|rs| {
                if predecessor.get(rs).is_none() {
                    if res {
                        return Err(());
                    } else {
                        res = true;
                    }
                }
                Ok(())
            })
            .is_ok()
    }

    /// Returns whether the automaton is strongly connected
    pub fn is_strongly_connected(&self) -> bool {
        self.kosaraju().len() <= 1
    }

    /// Returns whether the automaton is a orbit
    pub fn is_orbit(&self) -> bool {
        self.is_strongly_connected()
            && (self.states_count() != 1
                || unsafe { self.states().next().unwrap_unchecked() }
                    .as_ref()
                    .get_follows()
                    .any(|(_, set)| set.into_iter().any(|rs| self.states.contains(rs))))
    }

    /// Returns whether the orbit is stable
    pub fn is_stable(&self) -> bool {
        let mut inp = HashSet::new();
        let mut out = HashSet::new();
        let door = self.get_door();
        door.into_iter().for_each(|l| {
            l.into_iter().for_each(|(rs, dtype)| match dtype {
                DoorType::None => (),
                DoorType::In => {
                    inp.insert(rs);
                }
                DoorType::Out => {
                    out.insert(rs);
                }
                DoorType::Both => {
                    out.insert(rs.clone());
                    inp.insert(rs);
                }
            })
        });

        out.into_iter().all(|output| {
            output
                .as_ref()
                .get_follows()
                .any(|(_, set)| set.intersection(&inp).count() != 0)
        })
    }

    /// Returns whether the orbit is transverse
    pub fn is_transverse(&self) -> bool {
        let mut inp = HashSet::new();
        let mut out = HashSet::new();
        let mut fin = false;
        let mut fout = false;
        let door = self.get_door();
        door.into_iter()
            .try_for_each(|l| {
                l.into_iter().try_for_each(|(rs, dtype)| match dtype {
                    DoorType::None => Ok(()),
                    DoorType::In => {
                        if !fin {
                            rs.as_ref().get_previous().for_each(|(_, set)| {
                                set.into_iter().for_each(|rs| {
                                    inp.insert(rs.clone());
                                })
                            });
                            fin = true;
                            return Ok(());
                        }
                        fin = true;
                        rs.as_ref().get_previous().try_for_each(|(_, set)| {
                            set.into_iter().try_for_each(|rs| {
                                if inp.contains(rs) {
                                    Ok(())
                                } else {
                                    Err(())
                                }
                            })
                        })
                    }
                    DoorType::Out => {
                        if out.is_empty() {
                            rs.as_ref().get_follows().for_each(|(_, set)| {
                                set.into_iter().for_each(|rs| {
                                    out.insert(rs.clone());
                                })
                            });
                            fout = true;
                            return Ok(());
                        }
                        fout = true;
                        rs.as_ref().get_follows().try_for_each(|(_, set)| {
                            set.into_iter().try_for_each(|rs| {
                                if out.contains(rs) {
                                    Ok(())
                                } else {
                                    Err(())
                                }
                            })
                        })
                    }
                    DoorType::Both => {
                        if inp.is_empty() {
                            rs.as_ref().get_previous().for_each(|(_, set)| {
                                set.into_iter().for_each(|rs| {
                                    inp.insert(rs.clone());
                                })
                            });
                            fin = true;
                            return Ok(());
                        }
                        fin = true;
                        rs.as_ref().get_previous().try_for_each(|(_, set)| {
                            set.into_iter().try_for_each(|rs| {
                                if inp.contains(rs) {
                                    Ok(())
                                } else {
                                    Err(())
                                }
                            })
                        })?;
                        if out.is_empty() {
                            rs.as_ref().get_follows().for_each(|(_, set)| {
                                set.into_iter().for_each(|rs| {
                                    out.insert(rs.clone());
                                })
                            });
                            fout = true;
                            return Ok(());
                        }
                        fout = true;
                        rs.as_ref().get_follows().try_for_each(|(_, set)| {
                            set.into_iter().try_for_each(|rs| {
                                if out.contains(rs) {
                                    Ok(())
                                } else {
                                    Err(())
                                }
                            })
                        })
                    }
                })
            })
            .is_ok()
    }
}
