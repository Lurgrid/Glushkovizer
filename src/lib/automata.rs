//! Module permettant la gestion d'automate. Avec la possibilité de le
//! "crée à la main", de vérifié si un mot est reconnu par cet automate. Enfin
//! on peut aussi le convertir en [Graph], qui permettera une analyse sur
//! celui-ci et une représentation en dot.
//!
//! # Exemple
//!
//! Voici un exemple de l'utilisation d'un automate crée "à la main":
//! ```rust
//! use glushkovizer::automata::{Automata, FinitAutomata};
//! use petgraph::dot::Dot;
//!
//! fn main() {
//!     let mut g2: FinitAutomata<char> = FinitAutomata::new();
//!     g2.add_state();
//!     g2.add_state();
//!     g2.add_initial(0);
//!     g2.add_final(1);
//!     g2.add_transition(0, 1, 'a');
//!     println!(
//!         "L'automate reconnais le mot ?: {}",
//!         g2.accept(&("a".chars().collect::<Vec<char>>()[..]))
//!     );
//!     println!("{}", Dot::with_config(&g2.get_graph(), &[]));
//! }
//! ```
//!
//! Un autre exemple plus concret cette fois-ci, dans cet exemple on peut voir
//! qu'on "parse" une expression regulière puis on la convertie en automate pour
//! après reconnaitre des mots:
//! ```rust
//! use glushkovizer::automata::{Automata, FinitAutomata};
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
//!     let g = FinitAutomata::from(a);
//!     println!("{:?}", Dot::with_config(&g.get_graph(), &[]));
//!     println!(
//!         "L'automate reconnais le mot ?: {}",
//!         g.accept(&("ab".chars().collect::<Vec<char>>()[..]))
//!     );
//! }
//! ```

use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
pub mod glushkov;

/// Structure regroupant les informations nécessaire à la gestion d'un état d'un
/// autome.
struct State<T> {
    next: HashMap<T, Vec<usize>>,
}

/// Structure regroupant les informations nécessaire à la gestion d'un automate
/// finit.
pub struct FinitAutomata<T> {
    states: Vec<State<T>>,
    initials: Vec<usize>,
    finals: Vec<usize>,
}

/// Trait qui définie l'ensemble des méthodes disponnible sur un automate. Les
/// numéros d'états commence à 0 et s'incrémente à chaque ajout.
pub trait Automata<T> {
    /// Crée un automate initialement vide.
    fn new() -> Self;

    /// Test si le mot passé en paramètre est reconnu par l'automate.
    fn accept(&self, msg: &[T]) -> bool;

    /// Renvoie le nombre d'état dans l'automate
    fn get_nb_states(&self) -> usize;

    /// Renvoie la représentation de l'automate en graph.
    fn get_graph(&self) -> Graph<String, T>;

    /// Ajoute un état dans l'autome et retourne son numéro.
    fn add_state(&mut self) -> usize;

    /// Ajoute dans l'automate une transition de l'état de numéro "start" à
    ///     l'état de numéro "end" par le symbole "sym".
    ///     Renvoie vrai s'il y a reussi à ajouter la transition. Sinon renvoie
    ///     faux si l'un des états n'est pas dans l'automate.
    fn add_transition(&mut self, start: usize, end: usize, sym: T) -> bool;

    /// Ajoute "state" à la liste des états initials de l'automate.
    ///     Renvoie vrai si l'ajoute a été possible. Sinon renvoie faux si
    ///     l'états n'est pas dans l'automate.
    fn add_initial(&mut self, state: usize) -> bool;

    /// Ajoute "state" à la liste des états finaux de l'automate.
    ///     Renvoie vrai si l'ajoute a été possible. Sinon renvoie faux si
    ///     l'états n'est pas dans l'automate.
    fn add_final(&mut self, state: usize) -> bool;
}

impl<T> Automata<T> for FinitAutomata<T>
where
    T: PartialEq + Eq + Hash + Clone + Copy + Display,
{
    fn new() -> Self {
        FinitAutomata {
            states: Vec::new(),
            initials: Vec::new(),
            finals: Vec::new(),
        }
    }

    fn accept(&self, msg: &[T]) -> bool {
        let mut cur: Vec<usize> = self.initials.clone();
        for c in msg.iter() {
            let mut next: Vec<usize> = Vec::new();
            for s in cur.iter() {
                if let Some(l) = self.states[*s].next.get(&c) {
                    next.append(&mut l.clone())
                }
            }
            if next.is_empty() {
                return false;
            }
            cur = next;
        }
        cur.into_iter().find(|s| self.finals.contains(s)).is_some()
    }

    fn get_nb_states(&self) -> usize {
        self.states.len()
    }

    fn get_graph(&self) -> Graph<String, T> {
        let mut graph = Graph::<String, T>::new();
        for i in 0..self.states.len() {
            let mut l = String::new();
            if self.initials.contains(&i) {
                l.push_str("i");
            }
            if self.finals.contains(&i) {
                l.push_str("f");
            }
            l.push_str(i.to_string().as_str());
            graph.add_node(l);
        }
        for (i, s) in self.states.iter().enumerate() {
            for k in s.next.keys() {
                for v in s.next.get(k).unwrap() {
                    graph.add_edge(NodeIndex::new(i), NodeIndex::new(*v), *k);
                }
            }
        }
        graph
    }

    fn add_state(&mut self) -> usize {
        self.states.push(State {
            next: HashMap::new(),
        });
        self.states.len() - 1
    }

    fn add_transition(&mut self, start: usize, end: usize, sym: T) -> bool {
        if start >= self.states.len() {
            return false;
        }
        let s = &mut self.states[start];
        match s.next.get_mut(&sym) {
            Some(l) => {
                if !l.contains(&end) {
                    l.push(end)
                }
            }
            None => {
                s.next.insert(sym, vec![end]);
            }
        }
        true
    }

    fn add_initial(&mut self, state: usize) -> bool {
        if state >= self.states.len() {
            return false;
        }
        if !self.initials.contains(&state) {
            self.initials.push(state);
        }
        return true;
    }

    fn add_final(&mut self, state: usize) -> bool {
        if state >= self.states.len() {
            return false;
        }
        if !self.finals.contains(&state) {
            self.finals.push(state);
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::automata::{Automata, FinitAutomata};

    #[test]
    fn handmade() {
        let mut g = FinitAutomata::new();
        g.add_state();
        g.add_state();
        g.add_state();
        g.add_initial(0);
        g.add_final(2);
        g.add_transition(0, 1, 'a');
        g.add_transition(1, 2, 'a');
        assert_eq!(g.get_nb_states(), 3);
        assert!(g.accept(&['a', 'a']));
    }
}
