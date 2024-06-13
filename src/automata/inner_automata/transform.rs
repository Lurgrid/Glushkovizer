use super::{
    state::RefState,
    utils::{Couple, Epsilon, Union},
    InnerAutomata,
};
use std::{collections::HashSet, hash::Hash};

impl<'a, T, V> InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Creates a homogeneous automaton that recognizes the same language as the
    /// current automaton
    pub fn homogenize(&self) -> InnerAutomata<'a, T, Couple<Union<T, Epsilon>, V>> {
        let mut states = HashSet::default();
        let mut inputs = HashSet::default();
        let mut outputs = HashSet::default();

        self.states().for_each(|rs| {
            let mut empty = true;
            rs.as_ref().get_previous().for_each(|(sym, _)| {
                empty = false;
                let new_rs = RefState::new(Couple(
                    Union::left(sym.clone()),
                    rs.as_ref().get_value().clone(),
                ));
                let new_rs_clone = new_rs.clone();
                states.insert(new_rs);
                if self.is_input(rs) {
                    inputs.insert(new_rs_clone.clone());
                }
                if self.is_output(rs) {
                    outputs.insert(new_rs_clone);
                }
            });
            if empty {
                let new_rs = RefState::new(Couple(
                    Union::right(Epsilon),
                    rs.as_ref().get_value().clone(),
                ));
                let new_rs_clone = new_rs.clone();
                states.insert(new_rs);
                if self.is_input(rs) {
                    inputs.insert(new_rs_clone.clone());
                }
                if self.is_output(rs) {
                    outputs.insert(new_rs_clone);
                }
            }
        });

        let res = InnerAutomata::<T, Couple<Union<T, Epsilon>, V>> {
            states,
            inputs,
            outputs,
        };

        self.states().for_each(|from| {
            from.as_ref().get_follows().for_each(|(sym, set)| {
                set.into_iter().for_each(|to| {
                    let rs_to = unsafe {
                        res.states()
                            .find(|rs| {
                                &rs.as_ref().get_value().1 == to.as_ref().get_value()
                                    && if let Some(val) = rs.as_ref().get_value().0.get_left() {
                                        val == sym
                                    } else {
                                        false
                                    }
                            })
                            .unwrap_unchecked()
                    };

                    res.states().for_each(|rs_from| {
                        if &rs_from.as_ref().get_value().1 == from.as_ref().get_value() {
                            rs_from.add_follow(rs_to.clone(), sym.clone());
                        }
                    });
                })
            });
        });
        res
    }
}
