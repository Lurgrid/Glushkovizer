//! Module permettant la désérialisation en respectant l'invariant

use std::{collections::HashSet, hash::Hash};

use super::Automata;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

impl<T, V> Serialize for Automata<T, V>
where
    T: Eq + Hash + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self::serialize(self, serializer)
    }
}

#[allow(unconditional_recursion)]
impl<'de, T, V> Deserialize<'de> for Automata<T, V>
where
    T: Eq + Hash + Deserialize<'de>,
    V: Eq + Hash + Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        dbg!();
        let unchecked = Self::deserialize(deserializer)?;
        let mut initial: HashSet<V> = HashSet::new();
        if !unchecked.states.iter().all(|e| initial.insert(e.0.clone())) {
            return Err(de::Error::custom("Duplicate in initials"));
        }
        drop(initial);
        if !unchecked.finals.iter().all(|i| *i < unchecked.states.len()) {
            return Err(de::Error::custom("Unknow final state"));
        }
        if !unchecked
            .initials
            .iter()
            .all(|i| *i < unchecked.states.len())
        {
            return Err(de::Error::custom("Unknow initial state"));
        }
        if !unchecked.follow.iter().all(|hm| {
            hm.iter()
                .all(|(_, set)| set.iter().all(|i| *i < unchecked.states.len()))
        }) {
            return Err(de::Error::custom("Invalid follow"));
        }
        Ok(unchecked)
    }
}
