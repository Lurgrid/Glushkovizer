//! Module allowing automaton management. With the possibility of "created by
//! hand", checks if a word is recognized by this automata. Finally it can also
//! be converted into dot format
//!
//! # Examples
//!
//! Here is an example of using a hand-created automaton:
//! ```rust
//! use glushkovizer::automata::{error::Result, Automata};
//!
//! fn main() -> Result<()> {
//!     let mut g2 = Automata::new();
//!     g2.add_state(0);
//!     g2.add_state(1);
//!     g2.add_initial(0)?;
//!     g2.add_final(1)?;
//!     g2.add_transition(0, 1, 'a')?;
//!     println!(
//!         "L'automate reconnais le mot ?: {}",
//!         g2.accept("a".chars().collect::<Vec<char>>().iter())
//!     );
//!     println!("{}", g2);
//!     Ok(())
//! }
//! ```
//!
//! Another more concrete example: in this example we can see that we "parse" a
//! regular expression and then convert it into an automaton in order to
//! afterwards to recognize words:
//!
//! ```rust
//! use glushkovizer::automata::Automata;
//! use glushkovizer::regexp::RegExp;
//!
//! fn main() {
//!     let a = RegExp::try_from("(a+b).(a*.b)");
//!     if let Err(s) = a {
//!         eprintln!("Error ! {}", s);
//!         return;
//!     }
//!     let a = a.unwrap();
//!     let g = Automata::from(a);
//!     println!("{}", g);
//!     println!(
//!         "L'automate reconnais le mot ?: {}",
//!         g.accept("ab".chars().collect::<Vec<char>>().iter())
//!     );
//! }
//! ```

use crate::automata::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use self::error::AutomataError;

pub mod deserialize;
pub mod dfs;
pub mod display;
pub mod error;
pub mod glushkov;
pub mod in_out;
pub mod prop;
pub mod scc;

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Structure grouping together the information necessary for managing the state
/// of a automaton
struct State<V>(V);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Self")]
/// Structure grouping together the information necessary for managing an finite
/// automaton
pub struct Automata<T, V>
where
    T: Eq + Hash,
{
    states: Vec<State<V>>,
    initials: HashSet<usize>,
    finals: HashSet<usize>,
    follow: Vec<HashMap<T, HashSet<usize>>>,
}

impl<T, V> Default for Automata<T, V>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        Self {
            states: Default::default(),
            initials: Default::default(),
            finals: Default::default(),
            follow: Default::default(),
        }
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash,
{
    /// Returns the number of states in the automaton
    pub fn get_nb_states(&self) -> usize {
        self.states.len()
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Creates an initially empty automaton.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the following of the state with index "state" with transition
    /// "sym"
    ///
    /// No test is done on the validity of "state"
    unsafe fn follow_unchecked(&self, state: usize, sym: &T) -> Option<&HashSet<usize>> {
        self.follow[state].get(sym)
    }

    /// Returns the following states of the state which has the value "state"
    /// with for "sym" transition
    ///
    /// Returns an error if "state" is invalid
    pub fn follow(&self, state: &V, sym: &T) -> Result<Vec<V>> {
        let to = self
            .states
            .iter()
            .position(|s| s.0.eq(state))
            .ok_or(AutomataError::UnknowState)?;
        match unsafe { self.follow_unchecked(to, sym) } {
            None => Ok(vec![]),
            Some(set) => Ok(set.iter().map(|ind| self.states[*ind].0.clone()).collect()),
        }
    }

    /// Returns the following of the state with index "state" with path of
    /// "word" transition.
    ///
    /// No test is done on the validity of "state"
    unsafe fn follow_word_unchecked<'a>(
        &self,
        state: usize,
        word: impl Iterator<Item = &'a T>,
    ) -> HashSet<usize>
    where
        T: 'a,
    {
        word.fold(HashSet::from([state]), |cur, sym| {
            cur.into_iter()
                .filter_map(|ind| match self.follow_unchecked(ind, sym) {
                    None => None,
                    Some(set) => Some(set.clone()),
                })
                .fold(HashSet::new(), |mut acc, set| {
                    acc.extend(set);
                    acc
                })
        })
    }

    /// Returns the following of the state which has the value "state" with for
    /// transition path "word"
    ///
    /// Returns an error if "state" is invalid
    pub fn follow_word<'a>(&self, state: &V, word: impl Iterator<Item = &'a T>) -> Result<Vec<V>>
    where
        T: 'a,
    {
        let to = self
            .states
            .iter()
            .position(|s| s.0.eq(state))
            .ok_or(AutomataError::UnknowState)?;
        Ok(unsafe { self.follow_word_unchecked(to, word) }
            .into_iter()
            .map(|ind| self.states[ind].0.clone())
            .collect())
    }

    /// Test if the word passed as a parameter is recognized by the automaton
    pub fn accept<'a>(&self, word: impl Iterator<Item = &'a T> + Clone) -> bool
    where
        T: 'a,
    {
        self.initials
            .iter()
            .fold(HashSet::new(), |mut acc, &ind| {
                acc.extend(unsafe { self.follow_word_unchecked(ind, word.clone()) });
                acc
            })
            .into_iter()
            .find(|s| self.finals.contains(s))
            .is_some()
    }

    /// Returns the list of initial states
    pub fn get_initials(&self) -> Vec<V> {
        self.initials
            .iter()
            .map(|s| self.states[*s].0.clone())
            .collect()
    }

    /// Returns the list of final states
    pub fn get_finals(&self) -> Vec<V> {
        self.finals
            .iter()
            .map(|s| self.states[*s].0.clone())
            .collect()
    }

    /// Returns the list of states
    pub fn get_states(&self) -> Vec<V> {
        self.states.iter().map(|s| s.0.clone()).collect()
    }

    /// Returns the index of the state value "state"
    /// No test is done on the presence or absence of a state of this value
    unsafe fn get_ind_state(&self, state: &V) -> usize {
        self.states
            .iter()
            .position(|s| s.0.eq(state))
            .unwrap_unchecked()
    }

    /// Returns the inverse automaton, which therefore recognizes the mirror of
    /// the words
    pub fn get_inverse(&self) -> Self {
        let mut g = Self {
            states: self.states.clone(),
            initials: self.finals.clone(),
            finals: self.initials.clone(),
            follow: vec![HashMap::new(); self.get_nb_states()],
        };
        self.follow.iter().enumerate().for_each(|(from, follow)| {
            follow.iter().for_each(|(sym, set)| {
                set.into_iter().for_each(|to| unsafe {
                    Automata::add_transition_unchecked(&mut g, *to, from, sym.clone())
                });
            });
        });
        g
    }

    /// Creates a copy of the "sub-automaton", i.e. an automaton composed of
    /// of the "states" and having kept the transitions between these states.
    /// And having no initial and final states
    ///
    /// Returns an error if if "states" contains values that do not describe any
    /// state of the automaton state. Otherwise returns this copy of the
    /// sub-automaton
    pub fn get_subautomata(&self, states: &HashSet<V>) -> Result<Self> {
        if !states
            .iter()
            .all(|e| self.states.iter().find(|s| s.0.eq(e)).is_some())
        {
            return Err(AutomataError::UnknowState);
        }
        let mut a = Self::default();
        let mut npos = HashMap::new();
        states.into_iter().for_each(|v| {
            let oldp = unsafe { self.get_ind_state(v) };
            npos.insert(oldp, a.states.len());
            a.add_state(self.states[oldp].0.clone());
        });
        npos.iter().for_each(|(old_from, new_from)| {
            let follow = &self.follow[*old_from];
            follow.keys().for_each(|key| {
                let old_set = follow.get(key).unwrap();
                old_set.iter().for_each(|v| match npos.get(v) {
                    Some(new_to) => unsafe {
                        a.add_transition_unchecked(*new_from, *new_to, key.clone())
                    },
                    None => {}
                });
            });
        });
        Ok(a)
    }

    /// Adds a transition from the state of value "from" to the state of value
    /// "to" with "sym" as transition
    pub fn add_transition(&mut self, from: V, to: V, sym: T) -> Result<()> {
        let to = self
            .states
            .iter()
            .position(|s| s.0 == to)
            .ok_or(AutomataError::UnknowStateTo)?;
        let from = self
            .states
            .iter()
            .position(|s| s.0 == from)
            .ok_or(AutomataError::UnknowStateFrom)?;
        unsafe {
            self.add_transition_unchecked(from, to, sym);
        }
        Ok(())
    }

    /// Removes the transition from the "from" value state to the state of
    /// value "to" with "sym" as transition
    pub fn remove_transition(&mut self, from: V, to: V, sym: T) -> Result<()> {
        let to = self
            .states
            .iter()
            .position(|s| s.0 == to)
            .ok_or(AutomataError::UnknowStateTo)?;
        let from = self
            .states
            .iter()
            .position(|s| s.0 == from)
            .ok_or(AutomataError::UnknowStateFrom)?;
        unsafe {
            self.remove_transition_unchecked(from, to, sym);
        }
        Ok(())
    }

    /// Adds a transition from "from" index state to index state "to" with "sym"
    /// as transition
    ///
    /// No test is done if "from" and "to" are not valid indices
    unsafe fn add_transition_unchecked(&mut self, from: usize, to: usize, sym: T) {
        match self.follow[from].get_mut(&sym) {
            None => {
                self.follow[from].insert(sym, HashSet::from([to]));
            }
            Some(n) => {
                n.insert(to);
            }
        };
    }

    /// Remove transition from "from" index state to index state "to" with "sym"
    /// as transition
    ///
    /// No test is done if "from" and "to" are not valid indices
    unsafe fn remove_transition_unchecked(&mut self, from: usize, to: usize, sym: T) {
        match self.follow[from].get_mut(&sym) {
            Some(n) => {
                n.remove(&to);
            }
            _ => (),
        };
    }

    /// Adds a state to the state value automaton.
    ///
    /// Returns true if it was added and false if it was already present.
    pub fn add_state(&mut self, state: V) -> bool {
        if self.states.iter().find(|s| s.0 == state).is_some() {
            return false;
        }
        self.states.push(State(state));
        self.follow.push(HashMap::new());
        return true;
    }

    /// Removes the state from the automaton with the value "state"
    pub fn remove_state(&mut self, state: V) -> Result<()> {
        let s = self
            .states
            .iter()
            .position(|s| s.0 == state)
            .ok_or(AutomataError::UnknowState)?;
        self.states.remove(s);
        self.follow.remove(s);
        self.follow.iter_mut().for_each(|f| {
            f.values_mut().for_each(|set| {
                *set = set
                    .iter()
                    .filter_map(|ind| match *ind {
                        ind if ind == s => None,
                        ind if ind < s => Some(ind),
                        ind => Some(ind - 1),
                    })
                    .collect();
            });
        });
        Ok(())
    }

    /// Adds to the list of initial states of the automaton, the state which has
    /// the value "state"
    ///
    /// Returns true if it was added and false if it was already present
    pub fn add_initial(&mut self, state: V) -> Result<bool> {
        let s = self
            .states
            .iter()
            .position(|s| s.0 == state)
            .ok_or(AutomataError::UnknowState)?;
        Ok(self.initials.insert(s))
    }

    /// Deletes from the list of initial states of the automaton, the state
    /// whose value is "state"
    pub fn remove_initial(&mut self, state: V) -> Result<()> {
        let s = self
            .states
            .iter()
            .position(|s| s.0 == state)
            .ok_or(AutomataError::UnknowState)?;
        self.initials.remove(&s);
        Ok(())
    }

    /// Adds to the list of final states of the automaton, the state whose value
    /// is "state"
    ///
    /// Returns true if it was added and false if it was already present
    pub fn add_final(&mut self, state: V) -> Result<bool> {
        let s = self
            .states
            .iter()
            .position(|s| s.0 == state)
            .ok_or(AutomataError::UnknowState)?;
        Ok(self.finals.insert(s))
    }

    /// Deletes from the list of final states of the automaton, the state whose
    /// value is "state"
    pub fn remove_final(&mut self, state: V) -> Result<()> {
        let s = self
            .states
            .iter()
            .position(|s| s.0 == state)
            .ok_or(AutomataError::UnknowState)?;
        self.finals.remove(&s);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::automata::{error::Result, Automata};

    #[test]
    fn handmade() -> Result<()> {
        let mut g = Automata::new();
        g.add_state(1);
        g.add_state(2);
        g.add_state(3);
        g.add_state(4);
        g.add_initial(1)?;
        g.add_final(3)?;
        g.add_transition(1, 2, 'a')?;
        g.add_transition(2, 3, 'a')?;
        g.add_transition(3, 4, 'z')?;
        g.remove_state(4)?;
        assert_eq!(g.get_nb_states(), 3);
        assert!(g.accept(['a', 'a'].iter()));
        assert_eq!(g.follow_word(&1, ['a', 'z'].iter()).unwrap(), vec![]);
        assert!(!g.accept(['a', 'z'].iter()));
        Ok(())
    }

    #[test]
    fn inverse() -> Result<()> {
        let mut g = Automata::new();
        g.add_state(1);
        g.add_state(2);
        g.add_state(3);
        g.add_initial(1)?;
        g.add_final(3)?;
        g.add_transition(1, 2, 'a')?;
        g.add_transition(2, 3, 'b')?;
        let g = g.get_inverse();
        assert_eq!(g.get_nb_states(), 3);
        assert!(g.accept(['b', 'a'].iter()));
        Ok(())
    }

    #[test]
    fn subautomata() -> Result<()> {
        let mut g = Automata::new();
        g.add_state(1);
        g.add_state(2);
        g.add_state(3);
        g.add_initial(1)?;
        g.add_final(3)?;
        g.add_transition(1, 2, 'a')?;
        g.add_transition(2, 3, 'a')?;
        let g2 = g.get_subautomata(&[1, 2].into());
        assert!(g2.is_ok());
        let mut g2 = g2.unwrap();
        g2.add_initial(1)?;
        g2.add_final(2)?;
        assert_eq!(g2.get_nb_states(), 2);
        assert!(g2.accept(['a'].iter()));
        Ok(())
    }
}
