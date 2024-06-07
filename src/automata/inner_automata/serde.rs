use crate::automata::inner_automata::state::RefState;

use super::InnerAutomata;
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

impl<'a, T, V> Serialize for InnerAutomata<'a, T, V>
where
    T: Serialize + Eq + Hash + Clone,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Automata", 4)?;
        state.serialize_field(
            "states",
            &self
                .states
                .iter()
                .map(|rs| rs.as_ref().get_value())
                .collect::<Vec<&V>>(),
        )?;
        state.serialize_field(
            "inputs",
            &self
                .inputs
                .iter()
                .map(|rs| rs.as_ref().get_value())
                .collect::<Vec<&V>>(),
        )?;
        state.serialize_field(
            "outputs",
            &self
                .outputs
                .iter()
                .map(|rs| rs.as_ref().get_value())
                .collect::<Vec<&V>>(),
        )?;
        state.serialize_field(
            "follows",
            &self
                .states
                .iter()
                .fold(Vec::new(), |mut acc: Vec<(&V, &T, &V)>, from| {
                    from.as_ref().get_follows().for_each(|(symbol, set)| {
                        set.into_iter().for_each(|to| {
                            if self.states.contains(to) {
                                acc.push((
                                    from.as_ref().get_value(),
                                    symbol,
                                    to.as_ref().get_value(),
                                ));
                            }
                        });
                    });
                    acc
                }),
        )?;
        state.end()
    }
}

impl<'de, 'a, T, V> Deserialize<'de> for InnerAutomata<'a, T, V>
where
    T: Deserialize<'de> + Eq + Hash + Clone + 'a,
    V: Deserialize<'de> + Eq + 'a,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            States,
            Inputs,
            Outputs,
            Follows,
        }

        struct InnerAutomataVisitor<'a, T, V>
        where
            T: Eq + Hash + Clone,
            V: Eq,
        {
            phantom: PhantomData<&'a (T, V)>,
        }

        impl<'de, 'a, T, V> Visitor<'de> for InnerAutomataVisitor<'a, T, V>
        where
            T: Deserialize<'de> + Eq + Hash + Clone,
            V: Deserialize<'de> + Eq,
        {
            type Value = InnerAutomata<'a, T, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Automata")
            }

            fn visit_seq<W>(self, mut seq: W) -> Result<InnerAutomata<'a, T, V>, W::Error>
            where
                W: SeqAccess<'de>,
            {
                let states: Vec<V> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let inputs: Vec<V> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let outputs: Vec<V> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let follows: Vec<(V, T, V)> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let states: HashSet<RefState<'a, T, V>> = states
                    .into_iter()
                    .map(|state| RefState::new(state))
                    .collect();
                let inputs: HashSet<RefState<'a, T, V>> =
                    inputs
                        .into_iter()
                        .try_fold(HashSet::new(), |mut acc, input| {
                            match states.iter().find(|rs| rs.as_ref().get_value() == &input) {
                                None => Err(de::Error::custom("input not in states")),
                                Some(rs) => {
                                    acc.insert(rs.clone());
                                    Ok(acc)
                                }
                            }
                        })?;
                let outputs: HashSet<RefState<'a, T, V>> =
                    outputs
                        .into_iter()
                        .try_fold(HashSet::new(), |mut acc, output| {
                            match states.iter().find(|rs| rs.as_ref().get_value() == &output) {
                                None => Err(de::Error::custom("ouput not in states")),
                                Some(rs) => {
                                    acc.insert(rs.clone());
                                    Ok(acc)
                                }
                            }
                        })?;
                follows.into_iter().try_for_each(|(from, symbol, to)| {
                    let rto = match states.iter().find(|rs| rs.as_ref().get_value() == &to) {
                        None => Err(de::Error::custom("Unknown to state")),
                        Some(rs) => Ok(rs.clone()),
                    }?;
                    let rfrom = match states.iter().find(|rs| rs.as_ref().get_value() == &from) {
                        None => Err(de::Error::custom("Unknown from state")),
                        Some(rs) => Ok(rs),
                    }?;
                    rfrom.add_follow(rto, symbol);
                    Ok(())
                })?;
                Ok(InnerAutomata {
                    states,
                    inputs,
                    outputs,
                })
            }

            fn visit_map<W>(self, mut map: W) -> Result<InnerAutomata<'a, T, V>, W::Error>
            where
                W: MapAccess<'de>,
            {
                let mut field1 = None;
                let mut field2 = None;
                let mut field3 = None;
                let mut field4 = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::States => {
                            if field1.is_some() {
                                return Err(de::Error::duplicate_field("states"));
                            }
                            field1 = Some(map.next_value()?);
                        }
                        Field::Inputs => {
                            if field2.is_some() {
                                return Err(de::Error::duplicate_field("inputs"));
                            }
                            field2 = Some(map.next_value()?);
                        }
                        Field::Outputs => {
                            if field3.is_some() {
                                return Err(de::Error::duplicate_field("ouputs"));
                            }
                            field3 = Some(map.next_value()?);
                        }
                        Field::Follows => {
                            if field4.is_some() {
                                return Err(de::Error::duplicate_field("follows"));
                            }
                            field4 = Some(map.next_value()?);
                        }
                    }
                }
                let states: Vec<V> = field1.ok_or_else(|| de::Error::missing_field("states"))?;
                let inputs: Vec<V> = field2.ok_or_else(|| de::Error::missing_field("inputs"))?;
                let outputs: Vec<V> = field3.ok_or_else(|| de::Error::missing_field("outputs"))?;
                let follows: Vec<(V, T, V)> =
                    field4.ok_or_else(|| de::Error::missing_field("follows"))?;
                let states: HashSet<RefState<'a, T, V>> = states
                    .into_iter()
                    .map(|state| RefState::new(state))
                    .collect();
                let inputs: HashSet<RefState<'a, T, V>> =
                    inputs
                        .into_iter()
                        .try_fold(HashSet::new(), |mut acc, input| {
                            match states.iter().find(|rs| rs.as_ref().get_value() == &input) {
                                None => Err(de::Error::custom("input not in states")),
                                Some(rs) => {
                                    acc.insert(rs.clone());
                                    Ok(acc)
                                }
                            }
                        })?;
                let outputs: HashSet<RefState<'a, T, V>> =
                    outputs
                        .into_iter()
                        .try_fold(HashSet::new(), |mut acc, output| {
                            match states.iter().find(|rs| rs.as_ref().get_value() == &output) {
                                None => Err(de::Error::custom("ouput not in states")),
                                Some(rs) => {
                                    acc.insert(rs.clone());
                                    Ok(acc)
                                }
                            }
                        })?;
                follows.into_iter().try_for_each(|(from, symbol, to)| {
                    let rto = match states.iter().find(|rs| rs.as_ref().get_value() == &to) {
                        None => Err(de::Error::custom("Unknown to state")),
                        Some(rs) => Ok(rs.clone()),
                    }?;
                    let rfrom = match states.iter().find(|rs| rs.as_ref().get_value() == &from) {
                        None => Err(de::Error::custom("Unknown from state")),
                        Some(rs) => Ok(rs),
                    }?;
                    rfrom.add_follow(rto, symbol);
                    Ok(())
                })?;
                Ok(InnerAutomata {
                    states,
                    inputs,
                    outputs,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["states", "inputs", "outputs", "follows"];

        deserializer.deserialize_struct(
            "Automata",
            FIELDS,
            InnerAutomataVisitor {
                phantom: PhantomData,
            },
        )
    }
}
