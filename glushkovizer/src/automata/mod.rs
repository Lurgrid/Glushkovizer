//! Module allowing automaton management. With the possibility of "created by
//! hand", checks if a word is recognized by this automata. Finally it can also
//! be converted into dot format

pub mod error;
mod glushkov;
mod r#impl;
mod inner_automata;

use self::inner_automata::state::RefState;
pub use error::{AutomataError, Result};
use inner_automata::InnerAutomata;
pub use inner_automata::{dfs::DFSInfo, door::DoorType};
use r#impl::Inner;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::rc::{Rc, Weak};
use std::{
    cell::{RefCell, UnsafeCell},
    hash::Hash,
};

/// Data structure for parent automaton management
#[derive(Debug)]
struct InnerParent<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    inner: InnerAutomata<'a, T, V>,
    childs: Vec<Weak<RefCell<InnerAutomata<'a, T, V>>>>,
}

/// Data structure for automata management
#[derive(Debug)]
pub struct Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    himself: UnsafeCell<InnerParent<'a, T, V>>,
}

/// Data structure for managing a sub-automata
#[derive(Debug)]
pub struct SubAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    inner: Rc<RefCell<InnerAutomata<'a, T, V>>>,
    parent: *mut InnerParent<'a, T, V>,
}

/// Trait for retrieving state information
pub trait StatesInfo<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Returns the number of automata states
    fn states_count(&self) -> usize {
        self.inner().states_count()
    }

    /// Returns a list of all states
    fn states(&self) -> Vec<V> {
        self.inner()
            .states()
            .map(|r| Clone::clone(r.as_ref().get_value()))
            .collect()
    }
}

/// Trait for adding state to automaton
pub trait AddStates<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Adds a state to the set of states.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    fn add_state(&self, value: V) -> bool {
        let inner = self.inner();
        match inner.get_state(&value) {
            None => {
                let _ = inner;
                self.inner_mut().add_state(RefState::new(value))
            }
            Some(_) => false,
        }
    }
}

/// Trait for automaton state suppression
pub trait RemoveStates<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Removes a state from the set of states.
    ///
    /// Returns whether the state was present in the set.
    fn remove_state(&self, value: &V) -> Result<bool> {
        let inner = self.inner();
        match inner.get_state(value) {
            None => Err(AutomataError::UnknowState),
            Some(r) => Ok({
                let _ = inner;
                let inner = self.inner_mut();
                inner.remove_input(&r);
                inner.remove_output(&r);
                inner.remove_state(&r)
            }),
        }
    }
}

/// Trait for handling inputs/outputs
pub trait InOut<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Returns the number of inputs
    fn inputs_count(&self) -> usize {
        self.inner().inputs_count()
    }

    /// Returns the number of outputs
    fn outputs_count(&self) -> usize {
        self.inner().outputs_count()
    }

    /// Returns a list of all inputs states
    fn inputs(&self) -> Vec<V> {
        self.inner()
            .inputs()
            .map(|r| Clone::clone(r.as_ref().get_value()))
            .collect()
    }

    /// Returns a list of all ouputs states
    fn outputs(&self) -> Vec<V> {
        self.inner()
            .outputs()
            .map(|r| Clone::clone(r.as_ref().get_value()))
            .collect()
    }

    /// Adds a state to the set of inputs.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is
    ///     returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    fn add_input(&self, value: &V) -> Result<bool> {
        let inner = self.inner();
        match inner.get_state(value) {
            None => Err(AutomataError::UnknowState),
            Some(s) => Ok({
                let _ = inner;
                self.inner_mut().add_input(s)
            }),
        }
    }

    /// Adds a state to the set of outputs.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is
    ///     returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    fn add_output(&self, value: &V) -> Result<bool> {
        let inner = self.inner();
        match inner.get_state(value) {
            None => Err(AutomataError::UnknowState),
            Some(s) => Ok({
                let _ = inner;
                self.inner_mut().add_output(s)
            }),
        }
    }

    /// Removes a input from the set of states.
    ///
    /// Returns whether the input was present in the set.
    fn remove_input(&self, value: &V) -> Result<bool> {
        let inner = self.inner();
        match inner.get_state(value) {
            None => Err(AutomataError::UnknowState),
            Some(s) => Ok({
                let _ = inner;
                self.inner_mut().remove_input(&s)
            }),
        }
    }

    /// Removes a output from the set of states.
    ///
    /// Returns whether the output was present in the set.
    fn remove_output(&self, value: &V) -> Result<bool> {
        let inner = self.inner();
        match inner.get_state(value) {
            None => Err(AutomataError::UnknowState),
            Some(s) => Ok({
                let _ = inner;
                self.inner_mut().remove_output(&s)
            }),
        }
    }
}

/// Trait for transition information retrieval
pub trait TransitionInfo<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Returns the number of transitions out of the state
    fn transition_out_count(&self, state: &V) -> Result<usize> {
        self.inner()
            .get_state(state)
            .map(|s| s.get_transition_out_count())
            .ok_or(AutomataError::UnknowState)
    }

    /// Returns the number of incoming state transitions
    fn transition_in_count(&self, state: &V) -> Result<usize> {
        self.inner()
            .get_state(state)
            .map(|s| s.get_transition_in_count())
            .ok_or(AutomataError::UnknowState)
    }

    /// Returns the number of states with an incoming transition from "state" to
    /// it and with "symbol" as the symbol
    fn get_follow_count(&self, state: &V, symbol: &T) -> Result<usize> {
        self.inner()
            .get_state(state)
            .map(|s| s.get_follow_count(symbol))
            .ok_or(AutomataError::UnknowState)
    }

    /// Returns, the states with an incoming transition from "state" to them and
    /// with "symbol" as symbol
    fn get_follow(&self, state: &V, symbol: &T) -> Result<Vec<V>> {
        self.inner()
            .get_state(state)
            .map(|s| match s.as_ref().get_follow(symbol) {
                None => Vec::new(),
                Some(iterator) => iterator
                    .map(|res| res.as_ref().get_value().clone())
                    .collect(),
            })
            .ok_or(AutomataError::UnknowState)
    }

    /// Returns the list: symbol and state list representing transitions
    /// "state" outgoing
    fn get_follows(&self, state: &V) -> Result<Vec<(T, Vec<V>)>> {
        self.inner()
            .get_state(state)
            .map(|s| s.get_follows())
            .ok_or(AutomataError::UnknowState)
    }

    /// Returns the number of states with an incoming transition from it to
    /// "state" and with the symbol "symbol".
    fn get_previous_count(&self, state: &V, symbol: &T) -> Result<usize> {
        self.inner()
            .get_state(state)
            .map(|s| s.get_previous_count(symbol))
            .ok_or(AutomataError::NotEnoughState)
    }

    /// Returns, the states with an incoming transition from them to "state"
    /// and with "symbol" as symbol
    fn get_previou(&self, state: &V, symbol: &T) -> Result<Vec<V>> {
        self.inner()
            .get_state(state)
            .map(|s| match s.as_ref().get_previou(symbol) {
                None => Vec::new(),
                Some(iterator) => iterator
                    .map(|res| res.as_ref().get_value().clone())
                    .collect(),
            })
            .ok_or(AutomataError::UnknowState)
    }

    /// Returns the list: symbol and state list representing transitions
    /// "state" incoming
    fn get_previous(&self, state: &V) -> Result<Vec<(T, Vec<V>)>> {
        self.inner()
            .get_state(state)
            .map(|s| s.get_previous())
            .ok_or(AutomataError::UnknowState)
    }

    /// Returns the set of transition symbols from "from" to "to".
    fn get_transition(&self, from: &V, to: &V) -> Result<Vec<T>> {
        let sto = self
            .inner()
            .get_state(to)
            .ok_or(AutomataError::UnknowStateTo)?;

        self.inner()
            .get_state(from)
            .map(|state| {
                state
                    .as_ref()
                    .get_follows()
                    .filter_map(|(k, v)| match v.contains(&sto) {
                        true => Some(k.clone()),
                        false => None,
                    })
                    .collect()
            })
            .ok_or(AutomataError::UnknowStateFrom)
    }
}

/// Edit transitions trait
pub trait MutTransition<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Adds the successor "to" to the current state with the transition value
    /// "symbol"
    ///
    /// Indicates whether the transition has been newly added. That is:
    ///
    /// - If it didn't exist before, ```true``` is returned
    /// - If it already existed, ```false``` is returned, and the transition is
    ///     not modified: and the symbol passed as an argument is dropped.
    fn add_transition(&self, from: &V, to: &V, symbol: T) -> Result<bool> {
        let sto = self
            .inner()
            .get_state(to)
            .ok_or(AutomataError::UnknowStateTo)?;

        let sfrom = self
            .inner()
            .get_state(from)
            .ok_or(AutomataError::UnknowStateFrom)?;

        Ok(sfrom.add_follow(sto, symbol))
    }

    /// Deletes the "to" successor of the current state which had the "symbol"
    /// transition
    ///
    /// Returns if the transition existed before
    fn remove_transition(&self, from: &V, to: &V, symbol: &T) -> Result<bool> {
        let sto = self
            .inner()
            .get_state(to)
            .ok_or(AutomataError::UnknowStateTo)?;

        let sfrom = self
            .inner()
            .get_state(from)
            .ok_or(AutomataError::UnknowStateFrom)?;

        Ok(sfrom.remove_follow(&sto, symbol))
    }
}

///
pub trait Accept<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Returns ```true``` if the word is recognized by the automaton and
    /// ```false``` otherwise
    fn accept<'b>(&self, mut word: impl Iterator<Item = &'b T>) -> bool
    where
        'a: 'b,
        T: 'b,
    {
        match word.try_fold(
            self.inner().inputs().cloned().collect(),
            |start: Vec<RefState<T, V>>, symbol| {
                let mut temp: Vec<RefState<T, V>> = Vec::new();
                start.into_iter().for_each(|rs| {
                    if let Some(it) = rs.as_ref().get_follow(symbol) {
                        it.for_each(|rs| temp.push(rs.clone()));
                    }
                });
                if temp.is_empty() {
                    return Err(());
                }
                Ok(temp)
            },
        ) {
            Err(()) => false,
            Ok(it) => it.into_iter().any(|rs| self.inner().is_output(&rs)),
        }
    }
}

/// Trait allowing the copy of an automaton
pub trait Cloned<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Makes a copy of the automaton, removing transitions that are not in the
    /// automaton
    fn cloned(&self) -> Automata<'a, T, V> {
        Automata {
            himself: UnsafeCell::new(InnerParent {
                inner: self.inner().clone(),
                childs: Vec::default(),
            }),
        }
    }
}

/// Trait for automaton mirror calculation
pub trait Mirror<'a, T, V>: Inner<'a, T, V> + Cloned<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Creates a copy of the automaton and returns the mirror of this /
    /// automaton
    fn mirror(&self) -> Automata<'a, T, V> {
        let a = self.cloned();
        a.inner_mut().mirror();
        a
    }
}

/// Line defining the depth first search
pub trait DFS<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Returns the depth first search information. This route is done with the
    /// follows if "backward" is ```false``` and otherwise with the previous
    fn dfs(&self, order: Vec<V>, backward: bool) -> Result<DFSInfo<V>> {
        let order = order.into_iter().try_fold(Vec::new(), |mut acc, state| {
            match self.inner().get_state(&state) {
                None => Err(AutomataError::UnknowState),
                Some(rs) => {
                    if acc.contains(&rs) {
                        return Err(AutomataError::DuplicateState);
                    }
                    acc.push(rs);
                    Ok(acc)
                }
            }
        })?;
        let DFSInfo {
            prefix,
            suffix,
            predecessor,
        } = self.inner().dfs(order, backward);
        Ok(DFSInfo {
            prefix: prefix
                .into_iter()
                .map(|rs| rs.as_ref().get_value().clone())
                .collect(),
            suffix: suffix
                .into_iter()
                .map(|rs| rs.as_ref().get_value().clone())
                .collect(),
            predecessor: predecessor
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.as_ref().get_value().clone(),
                        v.as_ref().get_value().clone(),
                    )
                })
                .collect(),
        })
    }
}

/// kosaraju algorithm definition trait
pub trait Kosaraju<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Returns the result of the kosaraju algorithm on the automaton, i.e. the
    /// set of strongly connected components.
    fn kosaraju(&self) -> Vec<Vec<V>> {
        self.inner()
            .kosaraju()
            .into_iter()
            .map(|l| {
                l.into_iter()
                    .map(|rs| rs.as_ref().get_value().clone())
                    .collect()
            })
            .collect()
    }

    /// Returns the result of the kosaraju algorithm on the automaton, i.e. the
    /// set of strongly connected components. Where each element of the
    /// strongly connected components is a pair between the value of the state
    /// and the type of gate it is.
    fn kosaraju_type(&self) -> Vec<Vec<(V, DoorType)>> {
        self.inner()
            .get_door()
            .into_iter()
            .map(|l| {
                l.into_iter()
                    .map(|(rs, t)| (rs.as_ref().get_value().clone(), t))
                    .collect()
            })
            .collect()
    }
}

/// Line for DOT representation of the automaton
pub trait ToDot<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone + Display,
    V: Eq + Clone + Display,
{
    /// Returns the DOT representation of the automaton with inverted colors if
    /// "inverse" is ``true``
    fn to_dot(&self, inverse: bool) -> std::result::Result<String, std::fmt::Error> {
        self.inner().to_dot(inverse)
    }
}

/// Trait to define a method for extracting a sub-automaton from an automaton
pub trait ExtractSubAutomata<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Return the sub-automate composed of "state"" states with "inputs"" and
    /// "outputs" as inputs and outputs, respectively
    fn subautomata(
        &'a self,
        states: Vec<&V>,
        inputs: Vec<&V>,
        outputs: Vec<&V>,
    ) -> Result<SubAutomata<'a, T, V>>;
}

/// Trait for defining a method for extracting the sub-automata representing
/// the strongly connected components of an automaton
pub trait ExtractStronglyConnectedComponent<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Returns the sub-automata representing the strongly connected components
    /// of the automaton
    fn extract_scc(&'a self) -> Vec<SubAutomata<'a, T, V>>;
}

/// Trait for automaton property tests
pub trait Properties<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Returns if the automaton is standard
    fn is_standard(&self) -> bool {
        self.inner().is_standard()
    }

    /// Returns if the automaton is deterministic
    fn is_deterministic(&self) -> bool {
        self.inner().is_deterministic()
    }

    /// Returns if the automaton is fully deterministic
    fn is_fully_deterministic(&self) -> bool {
        self.inner().is_fully_deterministic()
    }

    /// Returns if the automaton is homogeneous
    fn is_homogeneous(&self) -> bool {
        self.inner().is_homogeneous()
    }

    /// Returns if the automaton is accessible
    fn is_accessible(&self) -> bool {
        self.inner().is_accessible()
    }

    /// Returns if the automaton is coaccessible
    fn is_coaccessible(&self) -> bool {
        self.inner().is_coaccessible()
    }

    /// Returns whether the automaton is strongly connected
    fn is_strongly_connected(&self) -> bool {
        self.inner().is_strongly_connected()
    }

    /// Returns whether the automaton is a orbit
    fn is_orbit(&self) -> bool {
        self.inner().is_orbit()
    }

    /// Returns whether the orbit is stable
    fn is_stable(&self) -> bool {
        self.inner().is_stable()
    }

    /// Returns whether the orbit is transverse
    fn is_transverse(&self) -> bool {
        self.inner().is_transverse()
    }
}
