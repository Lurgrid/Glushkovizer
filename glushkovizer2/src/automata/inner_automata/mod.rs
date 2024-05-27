//! Non-secure internal module for automata management
#![allow(dead_code)]

pub mod state;

use state::RefState;
use std::{collections::HashSet, hash::Hash};

/// Internal data structure for automaton management
#[derive(Clone, Debug)]
pub struct InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    states: HashSet<RefState<'a, T, V>>,
    inputs: HashSet<RefState<'a, T, V>>,
    outputs: HashSet<RefState<'a, T, V>>,
}

impl<'a, T, V> Default for InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self {
            states: Default::default(),
            inputs: Default::default(),
            outputs: Default::default(),
        }
    }
}

impl<'a, T, V> InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns an empty automaton
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of states
    pub fn states_count(&self) -> usize {
        self.states.len()
    }

    /// Returns the number of inputs
    pub fn inputs_count(&self) -> usize {
        self.inputs.len()
    }

    /// Returns the number of outputs
    pub fn outputs_count(&self) -> usize {
        self.outputs.len()
    }

    /// An iterator visiting all states in arbitrary order. The iterator
    /// element type is ``&RefState<T, V>``
    pub fn states(&self) -> impl Iterator<Item = &RefState<'a, T, V>> {
        self.states.iter()
    }

    /// An iterator visiting all inputs in arbitrary order. The iterator
    /// element type is ``&RefState<T, V>``
    pub fn inputs(&self) -> impl Iterator<Item = &RefState<'a, T, V>> {
        self.inputs.iter()
    }

    /// An iterator visiting all outputs in arbitrary order. The iterator
    /// element type is ``&RefState<T, V>``
    pub fn outputs(&self) -> impl Iterator<Item = &RefState<'a, T, V>> {
        self.outputs.iter()
    }

    /// Adds a state to the set of states.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    pub fn add_state(&mut self, value: RefState<'a, T, V>) -> bool {
        self.states.insert(value)
    }

    /// Adds a state to the set of inputs.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    pub fn add_input(&mut self, value: RefState<'a, T, V>) -> bool {
        self.inputs.insert(value)
    }

    /// Adds a state to the set of outputs.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    pub fn add_output(&mut self, value: RefState<'a, T, V>) -> bool {
        self.outputs.insert(value)
    }

    /// Removes a state from the set of states. Returns whether the state was
    /// present in the set.
    pub fn remove_state(&mut self, value: &RefState<'a, T, V>) -> bool {
        self.states.remove(value)
    }

    /// Removes a input from the set of states. Returns whether the input was
    /// present in the set.
    pub fn remove_input(&mut self, value: &RefState<'a, T, V>) -> bool {
        self.inputs.remove(value)
    }

    /// Removes a output from the set of states. Returns whether the output was
    /// present in the set.
    pub fn remove_output(&mut self, value: &RefState<'a, T, V>) -> bool {
        self.outputs.remove(value)
    }
}

impl<'a, T, V> InnerAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq,
{
    /// Returns the state reference with the value "value".
    pub fn get_state(&self, value: &V) -> Option<RefState<'a, T, V>> {
        self.states
            .iter()
            .find(|&r| r.as_ref().borrow().get_value().eq(value))
            .map(|r| r.clone())
    }

    /// Returns the input reference with the value "value".
    pub fn get_input(&self, value: &V) -> Option<RefState<'a, T, V>> {
        self.inputs
            .iter()
            .find(|&r| r.as_ref().borrow().get_value().eq(value))
            .map(|r| r.clone())
    }

    /// Returns the output reference with the value "value".
    pub fn get_output(&self, value: &V) -> Option<RefState<'a, T, V>> {
        self.outputs
            .iter()
            .find(|&r| r.as_ref().borrow().get_value().eq(value))
            .map(|r| r.clone())
    }
}
