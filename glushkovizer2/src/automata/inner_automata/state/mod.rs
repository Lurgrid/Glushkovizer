use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::{Rc, Weak},
};

use crate::set;

const WEAK_DEAD: &str = "The state died before the end of the use of this reference";

#[derive(Debug)]
pub enum RefState<T, V>
where
    T: Eq + Hash,
{
    StrongRefState(Rc<RefCell<State<T, V>>>),
    WeakRefState(Weak<RefCell<State<T, V>>>),
}

impl<T, V> PartialEq for RefState<T, V>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl<T, V> Eq for RefState<T, V> where T: Eq + Hash {}

impl<T, V> Hash for RefState<T, V>
where
    T: Eq + Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state);
    }
}

impl<T, V> AsRef<RefCell<State<T, V>>> for RefState<T, V>
where
    T: Eq + Hash,
{
    fn as_ref(&self) -> &RefCell<State<T, V>> {
        unsafe { &*self.as_ptr() }
    }
}

impl<T, V> Clone for RefState<T, V>
where
    T: Eq + Hash,
{
    fn clone(&self) -> Self {
        match self {
            Self::StrongRefState(r) => Self::WeakRefState(Rc::downgrade(r)),
            Self::WeakRefState(w) => Self::WeakRefState(Weak::clone(w)),
        }
    }
}

impl<T, V> From<Rc<RefCell<State<T, V>>>> for RefState<T, V>
where
    T: Eq + Hash,
{
    fn from(value: Rc<RefCell<State<T, V>>>) -> Self {
        Self::StrongRefState(value)
    }
}

impl<T, V> From<Weak<RefCell<State<T, V>>>> for RefState<T, V>
where
    T: Eq + Hash,
{
    fn from(value: Weak<RefCell<State<T, V>>>) -> Self {
        Self::WeakRefState(value)
    }
}

impl<T, V> RefState<T, V>
where
    T: Eq + Hash,
{
    pub fn new(value: V) -> Self {
        Self::StrongRefState(Rc::new(RefCell::new(State::new(value))))
    }

    pub fn as_ptr(&self) -> *const RefCell<State<T, V>> {
        match self {
            Self::StrongRefState(r) => Rc::as_ptr(r),
            Self::WeakRefState(r) => Rc::as_ptr(&r.upgrade().expect(WEAK_DEAD)),
        }
    }

    pub fn get_rc(&self) -> Rc<RefCell<State<T, V>>> {
        match self {
            Self::StrongRefState(r) => Rc::clone(r),
            Self::WeakRefState(w) => w.upgrade().expect(WEAK_DEAD),
        }
    }
}
#[derive(Clone, Debug)]
pub struct State<T, V>
where
    T: Eq + Hash,
{
    value: V,
    previous: HashMap<T, HashSet<RefState<T, V>>>,
    follow: HashMap<T, HashSet<RefState<T, V>>>,
}

impl<T, V> State<T, V>
where
    T: Eq + Hash,
{
    pub fn new(value: V) -> Self {
        Self {
            value,
            previous: Default::default(),
            follow: Default::default(),
        }
    }

    pub fn get_value(&self) -> &V {
        &self.value
    }
}

impl<T, V> RefState<T, V>
where
    T: Eq + Hash + Clone,
{
    pub fn add_follow(&self, to: RefState<T, V>, symbol: T) {
        let mut fmut = self.as_ref().borrow_mut();
        match fmut.follow.get_mut(&symbol) {
            None => {
                fmut.follow.insert(symbol.clone(), set![to.clone()]);
            }
            Some(set) => {
                set.insert(to.clone());
            }
        }
        drop(fmut);
        let mut tmut = to.as_ref().borrow_mut();
        match tmut.follow.get_mut(&symbol) {
            None => {
                tmut.follow.insert(symbol, set![self.clone()]);
            }
            Some(set) => {
                set.insert(self.clone());
            }
        }
    }
}
