//! Module regroupant les informations et implémentation nécéssaire à
//! la recuppération du type d'état d'un composant fortement connexe

use std::{collections::HashMap, hash::Hash, ops::AddAssign};

use super::Automata;

#[derive(PartialEq, Clone)]
/// Type qu'un état peut être
pub enum DoorType {
    /// Represente le fait que l'état est une porte d'entrée
    In,
    /// Represente le fait que l'état est une porte de sortie
    Out,
    /// Represente le fait que l'état est à la fois une porte d'entrée et une
    /// porte de sortie
    Both,
    /// Represente le fait que l'état n'est ni une porte d'entrée ni une porte
    /// de sortie
    None,
}

impl AddAssign<DoorType> for DoorType {
    fn add_assign(&mut self, rhs: DoorType) {
        *self = match self {
            &mut Self::In if rhs == Self::Out => Self::Both,
            &mut Self::Out if rhs == Self::In => Self::Both,
            &mut Self::None => rhs,
            _ => self.clone(),
        }
    }
}

impl<T, V> Automata<T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    /// Renvoie un tableau associatif entre un état et son type
    pub fn get_states_type(&self) -> HashMap<V, DoorType> {
        let k = self.kosaraju();
        let mut stype = HashMap::new();
        self.states.iter().for_each(|s| {
            stype.insert(s.0.clone(), DoorType::None);
        });
        for (ind, s) in self.states.iter().enumerate() {
            /*
            À débat, est-ce-que les initiaux sont des portes d'entrée
            if self.initials.contains(&ind) {
                *stype.get_mut(&s.0).unwrap() += DoorType::In;
            }
            */
            if self.finals.contains(&ind) {
                *stype.get_mut(&s.0).unwrap() += DoorType::Out;
            }
            for set in self.follow[ind].values() {
                for v in set {
                    let pf = k.iter().position(|vec| vec.contains(&s.0));
                    let pt = k.iter().position(|vec| vec.contains(&self.states[*v].0));
                    if pf != pt {
                        *stype.get_mut(&s.0).unwrap() += DoorType::Out;
                        *stype.get_mut(&self.states[*v].0).unwrap() += DoorType::In;
                    }
                }
            }
        }
        stype
    }
}
