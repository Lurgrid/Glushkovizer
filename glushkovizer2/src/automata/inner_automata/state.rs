//! Insecure internal module allowing the management of states and their
//! reference

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    pin::Pin,
};

/// Creates a HashSet with the given list of values
macro_rules! set {
    [ $x:expr ] => {
        {
            let mut y = HashSet::new();
            y.insert($x);
            y
        }
    };
    [ $($x:expr),+ ] => {
        HashSet::from([ $($x),+ ])
    };
}

/// Enumeration of possible state reference types
#[derive(Debug)]
pub enum RefState<'a, T, V>
where
    T: Eq + Hash,
{
    /// Represents strong state references
    StrongRefState(Pin<Box<State<'a, T, V>>>),
    /// Represents weak state references
    WeakRefState(*mut State<'a, T, V>),
}

impl<'a, T, V> PartialEq for RefState<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl<'a, T, V> Eq for RefState<'a, T, V> where T: Eq + Hash + Clone {}

impl<'a, T, V> Hash for RefState<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state);
    }
}

impl<'a, T, V> AsRef<State<'a, T, V>> for RefState<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    fn as_ref(&self) -> &State<'a, T, V> {
        unsafe { &*self.as_ptr() }
    }
}

impl<'a, T, V> Clone for RefState<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns a copy of the value
    ///
    /// Attention copy not deep, just copy the reference
    fn clone(&self) -> Self {
        Self::WeakRefState(self.as_ptr())
    }
}

impl<'a, T, V> RefState<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Creates a state reference with value "value"
    pub fn new(value: V) -> Self {
        Self::StrongRefState(Box::pin(State::new(value)))
    }

    /// Returns a raw pointer to the state [State]
    pub fn as_ptr(&self) -> *mut State<'a, T, V> {
        match self {
            Self::StrongRefState(r) => {
                (Pin::as_ref(r).get_ref() as *const State<'a, T, V>) as *mut State<'a, T, V>
            }
            Self::WeakRefState(r) => *r,
        }
    }

    fn as_mut(&self) -> &mut State<'a, T, V> {
        unsafe { &mut *self.as_ptr() }
    }
}

impl<'a, T, V> RefState<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Clone,
{
    /// Makes a deep copy of the referenced state and returns a strong
    /// reference to it
    pub fn cloned(&self) -> Self {
        Self::new(self.as_ref().value.clone())
    }

    /// Returns the list of symbol and [HashSet] pairs representing the
    /// follows of this state
    pub fn get_follows(&self) -> Vec<(T, Vec<V>)> {
        self.as_ref()
            .follow
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.into_iter().map(|r| r.as_ref().value.clone()).collect(),
                )
            })
            .collect()
    }

    /// Returns the list of symbol and [HashSet] pairs representing the
    /// previous of this state
    pub fn get_previous(&self) -> Vec<(T, Vec<V>)> {
        self.as_ref()
            .previous
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.into_iter().map(|r| r.as_ref().value.clone()).collect(),
                )
            })
            .collect()
    }
}

/// Data structure containing the information required to manage a automata
/// state
#[derive(Debug)]
pub struct State<'a, T, V>
where
    T: Eq + Hash,
{
    value: V,
    previous: HashMap<T, HashSet<RefState<'a, T, V>>>,
    follow: HashMap<T, HashSet<RefState<'a, T, V>>>,
}

impl<'a, T, V> State<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Creates a automaton state of value "value"
    pub fn new(value: V) -> Self {
        Self {
            value,
            previous: Default::default(),
            follow: Default::default(),
        }
    }

    /// Returns a reference to the internal state value
    pub fn get_value(&self) -> &V {
        &self.value
    }

    /// Returns the following of the current state with the transition "symbol"
    pub fn get_follow(&self, symbol: &T) -> Option<impl Iterator<Item = &RefState<'a, T, V>>> {
        self.follow.get(symbol).map(|s| s.iter())
    }

    /// Returns the previous of the current state with the transition "symbol"
    pub fn get_previou(&self, symbol: &T) -> Option<impl Iterator<Item = &RefState<'a, T, V>>> {
        self.previous.get(symbol).map(|s| s.iter())
    }

    /// Returns the set of symbol and follow pairs
    pub fn get_follows(&self) -> impl Iterator<Item = (&T, &HashSet<RefState<'a, T, V>>)> {
        self.follow.iter()
    }

    /// Returns the set of symbol and previous pairs
    pub fn get_previous(&self) -> impl Iterator<Item = (&T, &HashSet<RefState<'a, T, V>>)> {
        self.previous.iter()
    }

    /// Reverses state transitions
    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.follow, &mut self.previous);
    }
}

impl<'a, T, V> RefState<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns the number of next steps in the current state with transition
    /// "symbol"
    pub fn get_follow_count(&self, symbol: &T) -> usize {
        self.as_ref()
            .follow
            .get(symbol)
            .map(|s| s.len())
            .unwrap_or(0)
    }

    /// Returns the number of previous steps in the current state with
    /// transition "symbol"
    pub fn get_previous_count(&self, symbol: &T) -> usize {
        self.as_ref()
            .previous
            .get(symbol)
            .map(|s| s.len())
            .unwrap_or(0)
    }

    /// Returns the number of transitions out of state
    pub fn get_transition_out_count(&self) -> usize {
        self.as_ref().follow.len()
    }

    /// Returns the number of transitions in of state
    pub fn get_transition_in_count(&self) -> usize {
        self.as_ref().previous.len()
    }

    /// Returns current state transition symbols
    pub fn get_symbols(&self) -> Vec<T> {
        self.as_ref().follow.keys().cloned().collect()
    }

    /// Adds the successor "to" to the current state with the transition value
    /// "symbol"
    ///
    /// Indicates whether the transition has been newly added. That is:
    ///
    /// - If it didn't exist before, ```true``` is returned
    /// - If it already existed, ```false``` is returned, and the transition is
    ///     not modified: and the symbol passed as an argument is dropped.
    pub fn add_follow(&self, to: RefState<'a, T, V>, symbol: T) -> bool {
        let fmut = self.as_mut();
        match fmut.follow.get_mut(&symbol) {
            None => {
                fmut.follow.insert(symbol.clone(), set![to.clone()]);
            }
            Some(set) => {
                set.insert(to.clone());
            }
        }
        let tmut = to.as_mut();
        match tmut.previous.get_mut(&symbol) {
            None => {
                tmut.previous.insert(symbol, set![self.clone()]);
                false
            }
            Some(set) => {
                set.insert(self.clone());
                true
            }
        }
    }

    /// Deletes the "to" successor of the current state which had the
    /// "symbol" transition
    ///
    /// Returns if the transition existed before
    pub fn remove_follow(&self, to: &RefState<'a, T, V>, symbol: &T) -> bool {
        let fmut = self.as_mut();
        match fmut.follow.get_mut(&symbol) {
            None => {
                return false;
            }
            Some(set) => {
                set.remove(to);
                if set.is_empty() {
                    fmut.follow.remove(&symbol);
                }
            }
        }
        let tmut = to.as_mut();
        let set = unsafe { tmut.previous.get_mut(&symbol).unwrap_unchecked() };
        let res = set.remove(self);
        if set.is_empty() {
            tmut.previous.remove(&symbol);
        }
        res
    }

    /// Reverses referenced state transitions
    pub fn reverse(&self) {
        self.as_mut().reverse()
    }
}
