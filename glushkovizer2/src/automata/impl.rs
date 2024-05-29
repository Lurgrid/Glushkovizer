use super::*;
use std::{
    cell::{Ref, RefMut},
    hash::Hash,
};

const PARENT_DEAD: &'static str = "The parent died before the child";

/// Feature for internal automaton recupperation
pub trait Inner<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    /// Returns a reference to the internal automaton
    fn inner(&self) -> Ref<InnerAutomata<'a, T, V>>;

    /// Returns a mutable reference to the internal automaton
    fn inner_mut(&mut self) -> RefMut<InnerAutomata<'a, T, V>>;
}

pub trait Child<'a, T, V>
where
    T: Eq + Hash + Clone,
{
    fn parent(&self) -> &RefCell<InnerAutomata<'a, T, V>>;
}

impl<'a, T, V> Default for Automata<'a, T, V>
where
    T: Eq + Hash + Clone,
    V: Eq + Clone,
{
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerAutomata::default())),
            childs: Vec::default(),
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
    pub fn add_initial(&mut self, value: &V) -> Result<bool> {
        self.add_input(value)
    }

    /// Alias for [States::add_output()]
    pub fn add_final(&mut self, value: &V) -> Result<bool> {
        self.add_output(value)
    }

    /// Alias for [States::remove_input()]
    pub fn remove_initial(&mut self, value: &V) -> Result<bool> {
        self.remove_input(value)
    }

    /// Alias for [States::remove_output()]
    pub fn remove_final(&mut self, value: &V) -> Result<bool> {
        self.remove_output(value)
    }
}

macro_rules! inner {
    ($($t:ident),+) => {
        $(
            impl<'a, T, V> Inner<'a, T, V> for $t<'a, T, V>
            where
                T: Eq + Hash + Clone,
                V: Eq + Clone,
            {
                fn inner(&self) -> Ref<InnerAutomata<'a, T, V>> {
                    self.inner.as_ref().borrow()
                }

                fn inner_mut(&mut self) -> RefMut<InnerAutomata<'a, T, V>> {
                    self.inner.as_ref().borrow_mut()
                }
            }
        )+
    };
}

macro_rules! child {
    ($($t:ident),+) => {
        $(
            impl<'a, T, V> Child<'a, T, V> for $t<'a, T, V>
            where
                T: Eq + Hash + Clone,
                V: Eq + Clone,
            {
                fn parent(&self) -> &RefCell<InnerAutomata<'a, T, V>> {
                    unsafe { &*Rc::as_ptr(&self.parent.upgrade().expect(PARENT_DEAD)) }
                }
            }
        )+
    };
}

inner!(Automata, SubAutomata);
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
        fn add_state(&mut self, value: V) -> bool {
            let inner = self.parent().borrow();
            match inner.get_state(&value) {
                None => {
                    drop(inner);
                    let rs = RefState::new(value);
                    let rs2 = rs.clone();
                    self.parent().borrow_mut().add_state(rs);
                    self.inner_mut().add_state(rs2)
                }
                Some(rs) => {
                    drop(inner);
                    self.inner_mut().add_state(rs)
                },
            }
        }
    }
);

fimpl!(
    Automata => RemoveStates {
        fn remove_state(&mut self, value: &V) -> Result<bool> {
            let inner = self.inner();
            match inner.get_state(value) {
                None => Err(AutomataError::UnknowState),
                Some(r) => Ok({
                    inner.states().for_each(|state| {
                        state.get_symbols().into_iter().for_each(|ref symbol| {
                            state.remove_follow(&r, symbol);
                        })
                    });
                    drop(inner);
                    let mut inner_mut = self.inner_mut();
                    inner_mut.remove_input(&r);
                    inner_mut.remove_output(&r);
                    let res = inner_mut.remove_state(&r);
                    drop(inner_mut);
                    self.childs.retain(|c| {
                        if let Some(rc) = c.upgrade() {
                            let mut rcb = rc.borrow_mut();
                            rcb.remove_input(&r);
                            rcb.remove_output(&r);
                            rcb.remove_state(&r);
                            true
                        } else {
                            false
                        }
                    });
                    res
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
