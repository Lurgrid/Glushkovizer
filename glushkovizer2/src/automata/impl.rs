use super::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{cell::UnsafeCell, hash::Hash};

/// Feature for internal automaton recupperation
pub trait Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns a reference to the internal automaton
    fn inner(&self) -> &InnerAutomata<'a, T, V>;

    /// Returns a mutable reference to the internal automaton
    fn inner_mut(&self) -> &mut InnerAutomata<'a, T, V>;
}

impl<'a, T, V> Default for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn default() -> Self {
        Self {
            himself: UnsafeCell::new(InnerParent {
                inner: InnerAutomata::default(),
                childs: Vec::default(),
            }),
        }
    }
}

impl<'a, T, V> Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Creates an initially empty automaton
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, T, V> Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    /// Alias for [States::inputs_count()]
    pub fn initals_count(&self) -> usize {
        self.inputs_count()
    }

    /// Alias for [States::outputs_count()]
    pub fn finals_count(&self) -> usize {
        self.outputs_count()
    }

    /// Alias for [States::inputs()]
    pub fn initials(&self) -> Vec<V> {
        self.inputs()
    }

    /// Alias for [States::outputs()]
    pub fn finals(&self) -> Vec<V> {
        self.outputs()
    }

    /// Alias for [States::add_input()]
    pub fn add_initial(&self, value: &V) -> Result<bool> {
        self.add_input(value)
    }

    /// Alias for [States::add_output()]
    pub fn add_final(&self, value: &V) -> Result<bool> {
        self.add_output(value)
    }

    /// Alias for [States::remove_input()]
    pub fn remove_initial(&self, value: &V) -> Result<bool> {
        self.remove_input(value)
    }

    /// Alias for [States::remove_output()]
    pub fn remove_final(&self, value: &V) -> Result<bool> {
        self.remove_output(value)
    }
}

impl<'a, T, V> Inner<'a, T, V> for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn inner(&self) -> &InnerAutomata<'a, T, V> {
        &unsafe { &*self.himself.get() }.inner
    }

    fn inner_mut(&self) -> &mut InnerAutomata<'a, T, V> {
        &mut unsafe { &mut *self.himself.get() }.inner
    }
}

impl<'a, T, V> Inner<'a, T, V> for SubAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn inner(&self) -> &InnerAutomata<'a, T, V> {
        unsafe { &*self.inner.as_ptr() }
    }

    fn inner_mut(&self) -> &mut InnerAutomata<'a, T, V> {
        unsafe { &mut *self.inner.as_ptr() }
    }
}

macro_rules! fimpl {
    ($type:ident => $($trait:ident { $($code:item)* }),+) => {
        $(
            impl<'a, T, V> $trait<'a, T, V> for $type<'a, T, V>
            where
                T: Eq + Hash + Clone,
                V: Eq + Clone,
            {
                $($code)*
            }
        )+
    };
    ($type:ident, $($rest:ident),+ => $($trait:ident { $($code:item)* }),+) => {
        fimpl!($type => $($trait { $($code)* }),+);
        fimpl!($($rest),+ => $($trait { $($code)* }),+);
    };
}

macro_rules! derive {
    ($($type:ident),+ => $trait:ident) => {
        fimpl!($($type),+ => $trait {});
    };
    ($($type:ident),+ => $trait:ident, $($rest:ident),+) => {
        fimpl!($($type),+ => $trait {});
        derive!($($type),+ => $($rest),+);
    };
}

derive!(Automata, SubAutomata => StatesInfo, TransitionInfo, InOut, MutTransition, Accept, Cloned, Mirror, Kosaraju);
derive!(Automata => AddStates);
derive!(SubAutomata => RemoveStates);

fimpl!(
    SubAutomata => AddStates {
        fn add_state(&self, value: V) -> bool {
            let inner = &unsafe { & *self.parent }.inner;
            match inner.get_state(&value) {
                None => {
                    let rs = RefState::new(value);
                    let rs2 = rs.clone();
                    unsafe { &mut *self.parent }.inner.add_state(rs);
                    self.inner_mut().add_state(rs2)
                }
                Some(rs) => {
                    self.inner_mut().add_state(rs)
                },
            }
        }
    }
);

fimpl!(
    Automata => RemoveStates {
        fn remove_state(&self, value: &V) -> Result<bool> {
            let inner = self.inner();
            match inner.get_state(value) {
                None => Err(AutomataError::UnknowState),
                Some(r) => Ok({
                    let rb = r.as_ref();
                    let previous = rb.get_previous();
                    let follows = rb.get_follows();
                    previous.into_iter().for_each(|(symbol, set)| {
                        set.into_iter().for_each(|rs| {
                            rs.remove_follow(&r, &symbol);
                        });
                    });
                    follows.into_iter().for_each(|(symbol, set)| {
                        set.into_iter().for_each(|rs| {
                            r.remove_follow(&rs, &symbol);
                        });
                    });
                    unsafe { &mut *self.himself.get() }.childs.retain(|c| {
                        if let Some(rc) = c.upgrade() {
                            let mut rcb = rc.as_ref().borrow_mut();
                            rcb.remove_input(&r);
                            rcb.remove_output(&r);
                            rcb.remove_state(&r);
                            true
                        } else {
                            false
                        }
                    });
                    let inner_mut = self.inner_mut();
                    inner_mut.remove_input(&r);
                    inner_mut.remove_output(&r);
                    inner_mut.remove_state(&r)
                }),
            }
        }
    }
);

impl<'a, T, V, U> DFS<'a, T, V> for U
where
    T: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
    U: Inner<'a, T, V>,
{
}

impl<'a, T, V, U> ToDot<'a, T, V> for U
where
    T: Eq + Hash + Clone + Display,
    V: Eq + Clone + Display,
    U: Inner<'a, T, V>,
{
}

fimpl!(
    Automata => ExtractSubAutomata {
        fn subautomata(
            &'a self,
            states: Vec<&V>,
            inputs: Vec<&V>,
            outputs: Vec<&V>,
        ) -> Result<SubAutomata<'a, T, V>> {
            let inner = self.inner();
            let states = states
                .into_iter()
                .try_fold(HashSet::new(), |mut acc, state| {
                    let state = match inner.get_state(state) {
                        None => Err(AutomataError::UnknowState),
                        Some(rs) => Ok(rs),
                    }?;
                    if !acc.insert(state) {
                        return Err(AutomataError::DuplicateState);
                    }
                    Ok(acc)
                })?;
            let inputs = inputs
                .into_iter()
                .try_fold(HashSet::new(), |mut acc, input| {
                    let input = match inner.get_state(input) {
                        None => Err(AutomataError::UnknowState),
                        Some(rs) => Ok(rs),
                    }?;
                    if !states.contains(&input) {
                        return Err(AutomataError::InputStateIsNotInStates);
                    }
                    if !acc.insert(input) {
                        return Err(AutomataError::DuplicateState);
                    }
                    Ok(acc)
                })?;
            let outputs = outputs
                .into_iter()
                .try_fold(HashSet::new(), |mut acc, output| {
                    let output = match inner.get_state(output) {
                        None => Err(AutomataError::UnknowState),
                        Some(rs) => Ok(rs),
                    }?;
                    if !states.contains(&output) {
                        return Err(AutomataError::OutputStateIsNotInStates);
                    }
                    if !acc.insert(output) {
                        return Err(AutomataError::DuplicateState);
                    }
                    Ok(acc)
                })?;
            let r = Rc::new(RefCell::new(InnerAutomata::create(states, inputs, outputs)));
            unsafe { &mut *self.himself.get() }.childs.push(Rc::downgrade(&r));
            Ok(SubAutomata::<'a, T, V> {
                inner: r,
                parent: self.himself.get() as *mut InnerParent<'a, T, V>,
            })
        }
    }
);

fimpl!(
    SubAutomata => ExtractSubAutomata {
        fn subautomata(
            &'a self,
            states: Vec<&V>,
            inputs: Vec<&V>,
            outputs: Vec<&V>,
        ) -> Result<SubAutomata<'a, T, V>> {
            let inner = self.inner();
            let states = states
                .into_iter()
                .try_fold(HashSet::new(), |mut acc, state| {
                    let state = match inner.get_state(state) {
                        None => Err(AutomataError::UnknowState),
                        Some(rs) => Ok(rs),
                    }?;
                    if !acc.insert(state) {
                        return Err(AutomataError::DuplicateState);
                    }
                    Ok(acc)
                })?;
            let inputs = inputs
                .into_iter()
                .try_fold(HashSet::new(), |mut acc, input| {
                    let input = match inner.get_state(input) {
                        None => Err(AutomataError::UnknowState),
                        Some(rs) => Ok(rs),
                    }?;
                    if !states.contains(&input) {
                        return Err(AutomataError::InputStateIsNotInStates);
                    }
                    if !acc.insert(input) {
                        return Err(AutomataError::DuplicateState);
                    }
                    Ok(acc)
                })?;
            let outputs = outputs
                .into_iter()
                .try_fold(HashSet::new(), |mut acc, output| {
                    let output = match inner.get_state(output) {
                        None => Err(AutomataError::UnknowState),
                        Some(rs) => Ok(rs),
                    }?;
                    if !states.contains(&output) {
                        return Err(AutomataError::OutputStateIsNotInStates);
                    }
                    if !acc.insert(output) {
                        return Err(AutomataError::DuplicateState);
                    }
                    Ok(acc)
                })?;
            let r = Rc::new(RefCell::new(InnerAutomata::create(states, inputs, outputs)));
            unsafe {&mut *self.parent }.childs.push(Rc::downgrade(&r));
            Ok(SubAutomata::<'a, T, V> {
                inner: r,
                parent: self.parent
            })
        }
    }
);

fimpl!(
    Automata => ExtractStronglyConnectedComponent {
        fn extract_scc(&'a self) -> Vec<SubAutomata<'a, T, V>>
        {
            let childs = &mut unsafe {&mut *self.himself.get()}.childs;
            self.inner().extract_scc().into_iter().map(|inner| {
                let r = Rc::new(RefCell::new(inner));
                childs.push(Rc::downgrade(&r));
                SubAutomata::<'a, T, V> {
                    inner: r,
                    parent: self.himself.get() as *mut InnerParent<'a, T, V>
                }
            }).collect()
        }
    }
);

fimpl!(
    SubAutomata => ExtractStronglyConnectedComponent {
        fn extract_scc(&'a self) -> Vec<SubAutomata<'a, T, V>> {
            let childs = &mut unsafe { &mut *self.parent }.childs;

            self.inner()
                .extract_scc()
                .into_iter()
                .map(|inner| {
                    let r = Rc::new(RefCell::new(inner));
                    childs.push(Rc::downgrade(&r));
                    SubAutomata::<'a, T, V> {
                        inner: r,
                        parent: self.parent,
                    }
                })
                .collect()
        }
    }
);

impl<'a, T, V> Serialize for Automata<'a, T, V>
where
    T: Serialize + Eq + Hash + Clone,
    V: Serialize + Eq + Clone,
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unsafe { &*self.himself.get() }.serialize(serializer)
    }
}

impl<'de, 'a, T, V> Deserialize<'de> for Automata<'a, T, V>
where
    T: Deserialize<'de> + Eq + Hash + Clone + 'a,
    V: Deserialize<'de> + Eq + Clone + 'a,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let himself = InnerParent::deserialize(deserializer)?;
        Ok(Automata {
            himself: UnsafeCell::new(himself),
        })
    }
}

impl<'a, T, V> Serialize for InnerParent<'a, T, V>
where
    T: Serialize + Eq + Hash + Clone,
    V: Serialize + Eq + Clone,
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, 'a, T, V> Deserialize<'de> for InnerParent<'a, T, V>
where
    T: Deserialize<'de> + Eq + Hash + Clone + 'a,
    V: Deserialize<'de> + Eq + Clone + 'a,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let himself = InnerAutomata::deserialize(deserializer)?;
        Ok(InnerParent {
            inner: himself,
            childs: Vec::new(),
        })
    }
}

impl<'a, T, V> Serialize for SubAutomata<'a, T, V>
where
    T: Serialize + Eq + Hash + Clone,
    V: Serialize + Eq + Clone,
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.borrow().serialize(serializer)
    }
}
