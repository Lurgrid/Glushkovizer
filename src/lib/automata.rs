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
//! use petgraph::dot::Dot;
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
//!         g2.accept(&("a".chars().collect::<Vec<char>>()[..]))
//!     );
//!     println!("{}", Dot::with_config(&g2.get_graph(), &[]));
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
//! use petgraph::dot::Dot;
//!
//! fn main() {
//!     let a = RegExp::try_from("(a+b).(a*.b)");
//!     if let Err(s) = a {
//!         eprintln!("Error ! {}", s);
//!         return;
//!     }
//!     let a = a.unwrap();
//!     let g = Automata::from(a);
//!     println!("{:?}", Dot::with_config(&g.get_graph(), &[]));
//!     println!(
//!         "L'automate reconnais le mot ?: {}",
//!         g.accept(&("ab".chars().collect::<Vec<char>>()[..]))
//!     );
//! }
//! ```

use crate::automata::error::Result;
use petgraph::graph::{Graph, NodeIndex};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use self::error::AutomataError;

pub mod error;
pub mod glushkov;

/// Structure regroupant les informations nécessaire à la gestion d'un état d'un
/// automate.
pub struct State<T, V>
where
    V: Eq + Ord,
    T: Eq + Ord,
{
    value: V,
    next: BTreeMap<T, Vec<Rc<RefCell<State<T, V>>>>>,
}

impl<T, V> PartialOrd for State<T, V>
where
    V: Eq + Ord,
    T: Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, V> Ord for State<T, V>
where
    V: Eq + Ord,
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T, V> PartialEq for State<T, V>
where
    V: Eq + Ord,
    T: Eq + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, V> Eq for State<T, V>
where
    V: Eq + Ord,
    T: Eq + Ord,
{
}

impl<T, V> State<T, V>
where
    V: Eq + Ord + Clone + Copy,
    T: Eq + Ord,
{
    fn new(value: V) -> State<T, V> {
        State {
            value: value,
            next: Default::default(),
        }
    }
}

/// Structure regroupant les informations nécessaire à la gestion d'un automate
/// finit.
pub struct Automata<T, V>
where
    V: Eq + Ord,
    T: Eq + Ord,
{
    states: BTreeSet<Rc<RefCell<State<T, V>>>>,
    initials: BTreeSet<Rc<RefCell<State<T, V>>>>,
    finals: BTreeSet<Rc<RefCell<State<T, V>>>>,
}

impl<T, V> Default for Automata<T, V>
where
    V: Eq + Ord + Clone + Copy,
    T: Eq + Ord,
{
    fn default() -> Self {
        Self {
            states: Default::default(),
            initials: Default::default(),
            finals: Default::default(),
        }
    }
}

impl<T, V> Automata<T, V>
where
    V: Eq + Ord + Clone + Copy,
    T: Eq + Ord + Clone + Copy,
{
    /// Crée un automate initialement vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Test si le mot passé en paramètre est reconnu par l'automate.
    pub fn accept(&self, msg: &[T]) -> bool {
        let mut cur: Vec<Rc<RefCell<State<T, V>>>> = self.initials.clone().into_iter().collect();
        for c in msg.iter() {
            let mut next: Vec<Rc<RefCell<State<T, V>>>> = Vec::new();
            for s in cur.iter() {
                let s = s.as_ref();
                if let Some(l) = s.borrow().next.get(&c) {
                    for s in l.iter() {
                        next.push(s.clone())
                    }
                }
            }
            if next.is_empty() {
                return false;
            }
            cur = next;
        }
        cur.into_iter().find(|s| self.finals.contains(s)).is_some()
    }

    /// Renvoie le nombre d'état dans l'automate
    pub fn get_nb_states(&self) -> usize {
        self.states.len()
    }

    /// Renvoie la représentation de l'automate en [Graph]
    pub fn get_graph(&self) -> Graph<V, T> {
        let mut graph = Graph::new();
        for s in self.states.iter() {
            graph.add_node(s.as_ref().borrow().value.clone());
        }
        for (i, s) in self.states.iter().enumerate() {
            let s: &RefCell<State<T, V>> = s.borrow();
            let s = s.borrow();
            for k in s.next.keys() {
                for v in s.next.get(k).unwrap() {
                    graph.add_edge(
                        NodeIndex::new(i),
                        NodeIndex::new(self.states.iter().position(|x| x == v).unwrap()),
                        *k,
                    );
                }
            }
        }
        graph
    }

    /// Renvoie la liste des états initiaux.
    pub fn get_initials(&self) -> Vec<V> {
        self.initials
            .iter()
            .map(|s| s.as_ref().borrow().value.clone())
            .collect()
    }

    /// Renvoie la liste des états finaux.
    pub fn get_finals(&self) -> Vec<V> {
        self.finals
            .iter()
            .map(|s| s.as_ref().borrow().value.clone())
            .collect()
    }

    /// Renvoie la liste des états.
    pub fn get_states(&self) -> Vec<V> {
        self.states
            .iter()
            .map(|s| s.as_ref().borrow().value.clone())
            .collect()
    }

    /// Ajoute une transition entre l'état de valeur "from" vers l'état de
    /// valeur "to" avec comme transition "sym".
    /// Renvoie une erreur en cas d'impossibilité d'ajout et sinon un type unit.
    pub fn add_transition(&mut self, from: V, to: V, sym: T) -> Result<()> {
        let to = Rc::clone(
            self.states
                .get(&RefCell::new(State::new(to)))
                .ok_or(AutomataError::UnknowStateTo)?,
        );
        let from = self
            .states
            .get(&RefCell::new(State::new(from)))
            .ok_or(AutomataError::UnknowStateFrom)?;

        let f2 = from.as_ref().borrow();

        match f2.next.get(&sym) {
            None => {
                drop(f2);
                from.borrow_mut().next.insert(sym, vec![to]);
            }
            Some(n) if !n.contains(&to) => {
                drop(f2);
                let mut from = from.borrow_mut();
                let n = from.next.get_mut(&sym).unwrap();
                n.push(to);
            }
            _ => {}
        };
        Ok(())
    }

    /// Ajoute un état à l'automate de valeur "state".
    /// Renvoie vrai s'il a été ajouté et faux s'il était déjà présent.
    pub fn add_state(&mut self, state: V) -> bool {
        self.states.insert(Rc::new(RefCell::new(State::new(state))))
    }

    /// Ajoute à la liste des initaux de l'autome, l'état qui a pour valeur
    /// "state".
    /// Renvoie vrai s'il a été ajouté et faux s'il était déjà présent. Et
    /// renvoie une erreur si l'état n'existe pas.
    pub fn add_initial(&mut self, state: V) -> Result<bool> {
        let s = Rc::clone(
            self.states
                .get(&RefCell::new(State::new(state)))
                .ok_or(AutomataError::UnknowState)?,
        );
        Ok(self.initials.insert(s))
    }

    /// Ajoute à la liste des finaux de l'autome, l'état qui a pour valeur
    /// "state".
    /// Renvoie vrai s'il a été ajouté et faux s'il était déjà présent. Et
    /// renvoie une erreur si l'état n'existe pas.
    pub fn add_final(&mut self, state: V) -> Result<bool> {
        let s = Rc::clone(
            self.states
                .get(&RefCell::new(State::new(state)))
                .ok_or(AutomataError::UnknowState)?,
        );
        Ok(self.finals.insert(s))
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
        g.add_initial(1)?;
        g.add_final(3)?;
        g.add_transition(1, 2, 'a')?;
        g.add_transition(2, 3, 'a')?;
        assert_eq!(g.get_nb_states(), 3);
        assert!(g.accept(&['a', 'a']));
        Ok(())
    }
}