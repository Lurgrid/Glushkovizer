//! Sous module permettant la gestion d'automate

use petgraph::dot::Dot;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
pub mod glushkov;

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

/// Trait qui définie l'ensemble des méthodes disponnible sur un automate
pub trait Automata<T> {
    /// Crée un automate initialement vide.
    fn new() -> Self;

    /// Test si le mot passé en paramètre est reconnu par l'automate.
    fn accept(&self, msg: &[T]) -> bool;

    /// Renvoie le nombre d'état dans l'automate
    fn get_nb_states(&self) -> usize;

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

impl<T: PartialEq + Eq + Hash> Automata<T> for FinitAutomata<T> {
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

impl<T: PartialEq + Eq + Hash + Display> Display for FinitAutomata<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut graph = Graph::<usize, String>::new();
        for i in 0..self.states.len() {
            graph.add_node(i);
        }
        for (i, s) in self.states.iter().enumerate() {
            for k in s.next.keys() {
                for v in s.next.get(k).unwrap() {
                    graph.add_edge(NodeIndex::new(i), NodeIndex::new(*v), format!("{}{}", k, v));
                }
            }
        }
        write!(f, "{:?}", Dot::with_config(&graph, &[]))
    }
}
