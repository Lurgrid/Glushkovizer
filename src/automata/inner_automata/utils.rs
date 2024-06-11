use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Unit type representing epsilon
pub struct Epsilon;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Type representing a union between types T and V
pub enum Union<T, V> {
    /// Represents the case where the value comes from type T
    T(T),
    /// Represents the case where the value comes from type V
    V(V),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Type representing a couple
pub struct Couple<T, V>(pub T, pub V);

impl<T, V> Display for Union<T, V>
where
    T: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::T(t) => t.fmt(f),
            Self::V(v) => v.fmt(f),
        }
    }
}

impl<T, V> Display for Couple<T, V>
where
    T: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Display for Epsilon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", '\u{03F5}')
    }
}

impl<T, V> Union<T, V> {
    /// Creates a union with the T type value “value”.
    pub fn left(value: T) -> Self {
        Self::T(value)
    }

    /// Creates a union with the V type value “value”.
    pub fn right(value: V) -> Self {
        Self::V(value)
    }

    /// Returns true if the union value is of type T
    pub fn is_left(&self) -> bool {
        match self {
            Self::T(_) => true,
            Self::V(_) => false,
        }
    }

    /// Returns true if the union value is of type V
    pub fn is_right(&self) -> bool {
        match self {
            Self::T(_) => false,
            Self::V(_) => true,
        }
    }

    /// Returns the value of the union type T
    pub fn get_left(&self) -> Option<&T> {
        match self {
            Self::T(t) => Some(t),
            Self::V(_) => None,
        }
    }

    /// Returns the value of the union type V
    pub fn get_right(&self) -> Option<&V> {
        match self {
            Self::T(_) => None,
            Self::V(v) => Some(v),
        }
    }
}
