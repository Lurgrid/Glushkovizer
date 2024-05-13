//! Module regroupant les informations et implémentation nécéssaire au parcours
//! en profondeur.
//!
//! # Exemple
//! ```rust
//! use glushkovizer::automata::{Automata, error::Result};
//!
//! fn main() -> Result<()> {
//!     let mut g = Automata::new();
//!     g.add_state(0);
//!     g.add_state(1);
//!     g.add_state(2);
//!     g.add_transition(0, 1, 'a')?;
//!     g.add_transition(0, 2, 'a')?;
//!     g.add_transition(1, 0, 'b')?;
//!     g.add_transition(1, 2, 'b')?;
//!     g.add_transition(2, 0, 'c')?;
//!     g.add_transition(2, 1, 'c')?;
//!     let i = g.get_dfs(vec![1, 2, 3]);
//!     Ok(())
//! }
//! ```

use std::collections::HashSet;
use std::fmt::Debug;
use std::{hash::Hash, usize};

use super::error::{AutomataError, Result};
use super::Automata;

#[derive(Debug)]
/// Structure regroupant les informantions d'un parcours en profondeur de
/// l'automate
pub struct DFSInfo<V> {
    /// Ensemble d'état déﬁni par prefix\[i\] contient l'état que l’on
    /// découvre à l’instant i.
    pub prefix: Vec<V>,
    /// Ensemble d'état déﬁni par suffix\[i\] contient l'état que l’on termine
    /// d’explorer à l’instant i.
    pub suffix: Vec<V>,
    /// Ensemble, déﬁni par predecessor\[u\] = None, si u est une racine de la
    /// forêt d’exploration et par predecessor\[u\] est le prédécesseur du
    /// sommet u sinon
    pub predecessor: Vec<Option<V>>,
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Renvoie les informations du parcours en profondeur selon l'ordre
    /// "order".
    /// Renvoie [Err] si order ne contient par toute les valeurs possible des
    /// états.
    pub fn get_dfs(&self, order: Vec<V>) -> Result<DFSInfo<V>> {
        if self.get_nb_states() != order.len() || !self.states.iter().all(|d| order.contains(&d.0))
        {
            return Err(AutomataError::NotEnoughState);
        }

        let mut info = DFSInfo::<V> {
            prefix: Vec::with_capacity(self.get_nb_states()),
            suffix: Vec::with_capacity(self.get_nb_states()),
            predecessor: vec![None; self.get_nb_states()],
        };
        let mut color: Vec<bool> = vec![true; self.get_nb_states()];
        let mut state: Vec<usize> = (0..self.get_nb_states()).collect();
        state.sort_by_key(|ind| order.iter().position(|x| x.eq(&self.states[*ind].0)));
        for u in state {
            if color[u] {
                self.visit_in_depth(&order, u, &mut color, &mut info);
            }
        }
        Ok(info)
    }

    fn visit_in_depth(
        &self,
        order: &Vec<V>,
        u: usize,
        color: &mut Vec<bool>,
        info: &mut DFSInfo<V>,
    ) {
        color[u] = false;
        info.prefix.push(self.states[u].0.clone());
        let mut p: HashSet<usize> = HashSet::new();
        for set in self.follow[u].values() {
            p.extend(set);
        }
        let mut p: Vec<usize> = p.clone().into_iter().collect();
        p.sort_by_key(|ind| order.iter().position(|x| x.eq(&self.states[*ind].0)));
        for v in p {
            if color[v] {
                info.predecessor[v] = Some(self.states[u].0.clone());
                self.visit_in_depth(order, v, color, info)
            }
        }
        info.suffix.push(self.states[u].0.clone());
    }
}

#[cfg(test)]
mod test {
    use crate::automata::error::Result;
    use crate::automata::Automata;

    #[test]
    fn dfs() -> Result<()> {
        let mut g = Automata::new();
        g.add_state(0);
        g.add_state(1);
        g.add_state(2);
        g.add_state(3);
        g.add_transition(0, 1, 'a')?;
        g.add_transition(0, 2, 'a')?;
        g.add_transition(1, 0, 'b')?;
        g.add_transition(1, 2, 'b')?;
        g.add_transition(2, 0, 'c')?;
        g.add_transition(2, 1, 'c')?;
        g.add_transition(0, 3, 'd')?;
        let i = g.get_dfs(vec![0, 1, 2, 3]);
        assert!(i.is_ok());
        let i = i.unwrap();
        assert_eq!(i.prefix, vec![0, 1, 2, 3]);
        assert_eq!(i.suffix, vec![2, 1, 3, 0]);
        assert_eq!(i.predecessor, vec![None, Some(0), Some(1), Some(0)]);
        Ok(())
    }
}
