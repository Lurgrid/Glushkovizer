use super::*;
use std::hash::Hash;

const PARENT_DEAD: &'static str = "The parent died before the child";

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

pub trait Child<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn parent(&self) -> &RefCell<InnerParent<'a, T, V>>;
}

impl<'a, T, V> Default for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn default() -> Self {
        Self {
            himself: Rc::new(RefCell::new(InnerParent {
                inner: Rc::new(RefCell::new(InnerAutomata::default())),
                childs: Vec::default(),
            })),
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
        let inner_parent = unsafe { &*self.himself.as_ptr() };
        unsafe { &*inner_parent.inner.as_ptr() }
    }

    fn inner_mut(&self) -> &mut InnerAutomata<'a, T, V> {
        let inner_parent = unsafe { &mut *self.himself.as_ptr() };
        unsafe { &mut *inner_parent.inner.as_ptr() }
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

macro_rules! child {
    ($($t:ident),+) => {
        $(
            impl<'a, T, V> Child<'a, T, V> for $t<'a, T, V>
            where
                T: Eq + Hash + Clone,
                V: Eq + Clone,
            {
                fn parent(&self) -> &RefCell<InnerParent<'a, T, V>> {
                    unsafe { &*Rc::as_ptr(&self.parent.upgrade().expect(PARENT_DEAD)) }
                }
            }
        )+
    };
}

child!(SubAutomata);

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
            let inner_parent = self.parent().borrow();
            let inner = inner_parent.inner.as_ref().borrow();
            match inner.get_state(&value) {
                None => {
                    drop(inner);
                    drop(inner_parent);
                    let rs = RefState::new(value);
                    let rs2 = rs.clone();
                    self.parent().borrow_mut().inner.as_ref().borrow_mut().add_state(rs);
                    self.inner_mut().add_state(rs2)
                }
                Some(rs) => {
                    drop(inner);
                    drop(inner_parent);
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
                    let rb = r.as_ref().borrow();
                    let previous = rb.previous.clone();
                    let follows = rb.follow.clone();
                    drop(rb);
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
                    let _ = inner;
                    self.himself.as_ref().borrow_mut().childs.retain(|c| {
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
            let _ = inner;
            let r = Rc::new(RefCell::new(InnerAutomata::create(states, inputs, outputs)));
            self.himself.as_ref().borrow_mut().childs.push(Rc::downgrade(&r));
            Ok(SubAutomata::<'a, T, V> {
                inner: r,
                parent: Rc::downgrade(&self.himself),
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
            let _ = inner;
            let r = Rc::new(RefCell::new(InnerAutomata::create(states, inputs, outputs)));
            let parent = self.parent();
            parent.borrow_mut().childs.push(Rc::downgrade(&r));
            Ok(SubAutomata::<'a, T, V> {
                inner: r,
                parent: Rc::downgrade(&self.parent.upgrade().expect(PARENT_DEAD)),
            })
        }
    }
);

fimpl!(
    Automata => ExtractStronglyConnectedComponent {
        fn extract_scc(&'a self) -> Vec<SubAutomata<'a, T, V>>
        {
            let childs = &mut self.himself.borrow_mut().childs;
            self.inner().extract_scc().into_iter().map(|inner| {
                let r = Rc::new(RefCell::new(inner));
                childs.push(Rc::downgrade(&r));
                SubAutomata::<'a, T, V> {
                    inner: r,
                    parent: Rc::downgrade(&self.himself)
                }
            }).collect()
        }
    }
);

impl<'a, T, V> ExtractStronglyConnectedComponent<'a, T, V> for SubAutomata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn extract_scc(&'a self) -> Vec<SubAutomata<'a, T, V>> {
        let parent = self.parent.upgrade().expect(PARENT_DEAD);
        let childs = &mut parent.borrow_mut().childs;

        self.inner()
            .extract_scc()
            .into_iter()
            .map(|inner| {
                let r = Rc::new(RefCell::new(inner));
                childs.push(Rc::downgrade(&r));
                SubAutomata::<'a, T, V> {
                    inner: r,
                    parent: Rc::downgrade(&parent),
                }
            })
            .collect()
    }
}
