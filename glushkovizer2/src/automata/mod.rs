//! Module allowing automaton management. With the possibility of "created by
//! hand", checks if a word is recognized by this automata. Finally it can also
//! be converted into dot format

pub mod error;
mod inner_automata;

use self::inner_automata::state::RefState;
use error::{AutomataError, Result};
use inner::Inner;
use inner_automata::InnerAutomata;
use std::hash::Hash;

/// Data structure for automata management
#[derive(Debug)]
pub struct Automata<'a, T: Eq + Hash + Clone, V: Eq + Clone>(InnerAutomata<'a, T, V>);

impl<'a, T, V> Default for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn default() -> Self {
        Self(InnerAutomata::default())
    }
}

impl<'a, T, V> Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Creates an initially empty automaton
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, T, V> States<'a, T, V> for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn remove_state(&mut self, value: &V) -> Result<bool> {
        self.inner()
            .get_state(value)
            .map(|ref r| {
                if !self.inner_mut().remove_state(r) {
                    return false;
                }
                self.inner().states().for_each(|state| {
                    state.get_symbols().into_iter().for_each(|ref symbol| {
                        state.remove_follow(r, symbol);
                    })
                });
                true
            })
            .ok_or(AutomataError::UnknowState)
    }
}

impl<'a, T, V> Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Alias for [States::inputs_count()]
    pub fn initals_count(&self) -> usize {
        self.inputs_count()
    }

    /// Alias for [States::outputs_count()]
    pub fn finals_count(&self) -> usize {
        self.outputs_count()
    }

    /// Alias for [States::inputs()]
    pub fn initials(&self) -> Vec<V> {
        self.inputs()
    }

    /// Alias for [States::outputs()]
    pub fn finals(&self) -> Vec<V> {
        self.outputs()
    }

    /// Alias for [States::add_input()]
    pub fn add_initial(&mut self, value: &V) -> Result<bool> {
        self.add_input(value)
    }

    /// Alias for [States::add_output()]
    pub fn add_final(&mut self, value: &V) -> Result<bool> {
        self.add_output(value)
    }

    /// Alias for [States::remove_input()]
    pub fn remove_initial(&mut self, value: &V) -> Result<bool> {
        self.remove_input(value)
    }

    /// Alias for [States::remove_output()]
    pub fn remove_final(&mut self, value: &V) -> Result<bool> {
        self.remove_output(value)
    }
}

impl<'a, T, V> TransitionInfo<'a, T, V> for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
}

impl<'a, T, V> MutTransition<'a, T, V> for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
}

impl<'a, T, V> Cloned<'a, T, V> for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
}

/// Data structure for managing a sub-automata
#[derive(Debug)]
pub struct SubAutomata<'a, T: Eq + Hash + Clone, V: Eq>(InnerAutomata<'a, T, V>);

impl<'a, T, V> States<'a, T, V> for SubAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
}

impl<'a, T, V> TransitionInfo<'a, T, V> for SubAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
}

impl<'a, T, V> Cloned<'a, T, V> for SubAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
}

mod inner {
    use super::inner_automata::InnerAutomata;
    use std::hash::Hash;
    /// Feature for internal automaton recupperation
    pub trait Inner<'a, T, V>
    where
        T: Eq + Hash + Clone,
    {
        /// Returns a reference to the internal automaton
        fn inner(&self) -> &InnerAutomata<'a, T, V>;

        /// Returns a mutable reference to the internal automaton
        fn inner_mut(&mut self) -> &mut InnerAutomata<'a, T, V>;
    }
}

macro_rules! automata {
    ($t:ident) => {
        impl<'a, T, V> Inner<'a, T, V> for $t<'a, T, V>
        where
            T: Eq + Hash + Clone,
            V: Eq + Clone,
        {
            fn inner(&self) -> &InnerAutomata<'a, T, V> {
                &self.0
            }

            fn inner_mut(&mut self) -> &mut InnerAutomata<'a, T, V> {
                &mut self.0
            }
        }
    };
}

automata!(Automata);
automata!(SubAutomata);

/// Trait for all state methods
pub trait States<'a, T, V>: Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Returns the number of automata states
    fn states_count(&self) -> usize {
        self.inner().states_count()
    }

    /// Returns the number of inputs
    fn inputs_count(&self) -> usize {
        self.inner().inputs_count()
    }

    /// Returns the number of outputs
    fn outputs_count(&self) -> usize {
        self.inner().outputs_count()
    }

    /// Returns a list of all states
    fn states(&self) -> Vec<V> {
        self.inner()
            .states()
            .map(|r| Clone::clone(r.as_ref().borrow().get_value()))
            .collect()
    }

    /// Returns a list of all inputs states
    fn inputs(&self) -> Vec<V> {
        self.inner()
            .inputs()
            .map(|r| Clone::clone(r.as_ref().borrow().get_value()))
            .collect()
    }

    /// Returns a list of all ouputs states
    fn outputs(&self) -> Vec<V> {
        self.inner()
            .outputs()
            .map(|r| Clone::clone(r.as_ref().borrow().get_value()))
            .collect()
    }

    /// Adds a state to the set of states.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    fn add_state(&mut self, value: V) -> bool {
        match self.inner().get_state(&value) {
            None => self.inner_mut().add_state(RefState::new(value)),
            Some(_) => false,
        }
    }

    /// Adds a state to the set of inputs.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    fn add_input(&mut self, value: &V) -> Result<bool> {
        self.inner()
            .get_state(value)
            .map(|s| self.inner_mut().add_input(s))
            .ok_or(AutomataError::UnknowState)
    }

    /// Adds a state to the set of outputs.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this state, ``true`` is returned
    /// - If the set already contained this state, ``false`` is returned, and
    ///     the set is not modified: original state is not replaced, and the
    ///     state passed as argument is dropped
    fn add_output(&mut self, value: &V) -> Result<bool> {
        self.inner()
            .get_state(value)
            .map(|s| self.inner_mut().add_output(s))
            .ok_or(AutomataError::UnknowState)
    }

    /// Removes a state from the set of states.
    ///
    /// Returns whether the state was present in the set.
    fn remove_state(&mut self, value: &V) -> Result<bool> {
        self.inner()
            .get_state(value)
            .map(|r| self.inner_mut().remove_state(&r))
            .ok_or(AutomataError::UnknowState)
    }

    /// Removes a input from the set of states.
    ///
    /// Returns whether the input was present in the set.
    fn remove_input(&mut self, value: &V) -> Result<bool> {
        self.inner()
            .get_state(value)
            .map(|r| self.inner_mut().remove_input(&r))
            .ok_or(AutomataError::UnknowState)
    }

    /// Removes a output from the set of states.
    ///
    /// Returns whether the output was present in the set.
    fn remove_output(&mut self, value: &V) -> Result<bool> {
        self.inner()
            .get_state(value)
            .map(|r| self.inner_mut().remove_output(&r))
            .ok_or(AutomataError::UnknowState)
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
                    .borrow()
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
    fn add_transition(&mut self, from: &V, to: &V, symbol: T) -> Result<bool> {
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
    fn remove_transition(&mut self, from: &V, to: &V, symbol: &T) -> Result<bool> {
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

/// Trait allowing the copy of an automaton
pub trait Cloned<'a, T, V>: Inner<'a, T, V> + TransitionInfo<'a, T, V> + States<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Makes a copy of the automaton, removing transitions that are not in the
    /// automaton
    fn cloned(&self) -> Automata<'a, T, V> {
        let mut automata: Automata<T, V> = Automata::new();

        let states = self.states();

        states.iter().for_each(|state| {
            automata.add_state(state.clone());
        });

        states.into_iter().for_each(|from| {
            automata.get_follows(&from).into_iter().for_each(|list| {
                list.into_iter().for_each(|(symbol, s)| {
                    s.into_iter().for_each(|to| {
                        let _ = automata.add_transition(&from, &to, symbol.clone());
                    })
                })
            })
        });
        automata
    }
}
