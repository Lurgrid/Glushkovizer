//! Module permettant la gestion d'automate. Avec la possibilité de le
//! "crée à la main", de vérifié si un mot est reconnu par cet automate. Enfin
//! on peut aussi le convertir en [Graph], qui permettera une analyse sur
//! celui-ci et une représentation en dot.
//!
//! # Exemple
//!
//! Voici un exemple de l'utilisation d'un automate crée "à la main":
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
//! Un autre exemple plus concret cette fois-ci, dans cet exemple on peut voir
//! qu'on "parse" une expression regulière puis on la convertie en automate pour
//! après reconnaitre des mots:
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
/// Structure regroupant les informations nécessaire à la gestion d'un état d'un
/// automate.
struct State<V>(V);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(remote = "Self")]
/// Structure regroupant les informations nécessaire à la gestion d'un automate
/// finit.
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
    /// Renvoie le nombre d'état dans l'automate
    pub fn get_nb_states(&self) -> usize {
        self.states.len()
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Crée un automate initialement vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Renvoie les suivants de l'état d'indice "state" avec pour transition
    /// "sym".
    /// Aucun test n'est fait sur la validiter de "state"
    unsafe fn follow_unchecked(&self, state: usize, sym: &T) -> Option<&HashSet<usize>> {
        self.follow[state].get(sym)
    }

    /// Renvoie les états suivant de l'état qui a pour valeur "state" avec pour
    /// transition "sym".
    /// Renvoie une erreur si "state" n'est pas valide.
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

    /// Renvoie les suivants de l'état d'indice "state" avec pour chemain de
    /// transition "word".
    /// Aucun test n'est fait sur la validiter de "state"
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

    /// Renvoie les suivants de l'état qui à pour valeur "state" avec pour
    /// chemain de transition "word".
    /// Renvoie une erreur si "state" n'est pas valide.
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

    /// Test si le mot passé en paramètre est reconnu par l'automate.
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

    /// Renvoie la liste des états initiaux.
    pub fn get_initials(&self) -> Vec<V> {
        self.initials
            .iter()
            .map(|s| self.states[*s].0.clone())
            .collect()
    }

    /// Renvoie la liste des états finaux.
    pub fn get_finals(&self) -> Vec<V> {
        self.finals
            .iter()
            .map(|s| self.states[*s].0.clone())
            .collect()
    }

    /// Renvoie la liste des états.
    pub fn get_states(&self) -> Vec<V> {
        self.states.iter().map(|s| s.0.clone()).collect()
    }

    /// Renvoie l'indice de l'état de valeur "state".
    /// Aucun test n'est fait sur la présence ou non d'un état de cette valeur
    unsafe fn get_ind_state(&self, state: &V) -> usize {
        self.states
            .iter()
            .position(|s| s.0.eq(state))
            .unwrap_unchecked()
    }

    /// Renvoie l'automate inverse, qui reconnait donc le miroir des mots.
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

    /// Crée une copie du "sous automate", c'est à dire un automate composé
    /// des états "states" et ayant gardé les transition entre ces états. Et
    /// ayant aucun états initials et finaux
    /// Renvoie une erreur si les valeurs de "states" contient des doublons ou
    /// si "states" contient des valeurs ne décrivant aucun état de l'automate
    /// courrant. Sinon renvoie cette copie du sous automate.
    pub fn get_subautomata(&self, states: &Vec<V>) -> Result<Self> {
        if has_dup(&states) {
            return Err(AutomataError::DuplicateState);
        }
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

    /// Ajoute une transition entre l'état de valeur "from" vers l'état de
    /// valeur "to" avec comme transition "sym".
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

    /// Supprime la transition entre l'état de valeur "from" vers l'état de
    /// valeur "to" avec comme transition "sym".
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

    /// Ajoute une transition entre l'état d'indice "from" vers l'état d'indice
    /// "to" avec comme transition "sym".
    /// Aucun test n'est fait si "from" et "to" ne sont pas des indices valides
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

    /// Supprime la transition entre l'état d'indice "from" vers l'état d'indice
    /// "to" avec comme transition "sym".
    /// Aucun test n'est fait si "from" et "to" ne sont pas des indices valides
    unsafe fn remove_transition_unchecked(&mut self, from: usize, to: usize, sym: T) {
        match self.follow[from].get_mut(&sym) {
            Some(n) => {
                n.remove(&to);
            }
            _ => (),
        };
    }

    /// Ajoute un état à l'automate de valeur "state".
    /// Renvoie vrai s'il a été ajouté et faux s'il était déjà présent.
    pub fn add_state(&mut self, state: V) -> bool {
        if self.states.iter().find(|s| s.0 == state).is_some() {
            return false;
        }
        self.states.push(State(state));
        self.follow.push(HashMap::new());
        return true;
    }

    /// Supprime le état à l'automate de valeur "state".
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

    /// Ajoute à la liste des initiaux de l'automate, l'état qui a pour valeur
    /// "state".
    /// Renvoie vrai s'il a été ajouté et faux s'il était déjà présent.
    pub fn add_initial(&mut self, state: V) -> Result<bool> {
        let s = self
            .states
            .iter()
            .position(|s| s.0 == state)
            .ok_or(AutomataError::UnknowState)?;
        Ok(self.initials.insert(s))
    }

    /// Supprime à la liste des initiaux de l'automate, l'état qui a pour valeur
    /// "state".
    pub fn remove_initial(&mut self, state: V) -> Result<()> {
        let s = self
            .states
            .iter()
            .position(|s| s.0 == state)
            .ok_or(AutomataError::UnknowState)?;
        self.initials.remove(&s);
        Ok(())
    }

    /// Ajoute à la liste des finaux de l'automate, l'état qui a pour valeur
    /// "state".
    /// Renvoie vrai s'il a été ajouté et faux s'il était déjà présent.
    pub fn add_final(&mut self, state: V) -> Result<bool> {
        let s = self
            .states
            .iter()
            .position(|s| s.0 == state)
            .ok_or(AutomataError::UnknowState)?;
        Ok(self.finals.insert(s))
    }

    /// Supprime à la liste des finaux de l'automate, l'état qui a pour valeur
    /// "state".
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

fn has_dup<T: Eq + Hash + Clone>(vec: &Vec<T>) -> bool {
    let mut set: HashSet<T> = HashSet::new();
    !vec.iter().all(|e| set.insert(e.clone()))
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
        let g2 = g.get_subautomata(&vec![1, 2]);
        assert!(g2.is_ok());
        let mut g2 = g2.unwrap();
        g2.add_initial(1)?;
        g2.add_final(2)?;
        assert_eq!(g2.get_nb_states(), 2);
        assert!(g2.accept(['a'].iter()));
        Ok(())
    }
}
