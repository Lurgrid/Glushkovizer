//! Traits and essential types intended for blanket imports

#[macro_export]
/// Creates a HashSet with the given list of values
macro_rules! set {
    [ $x:expr ] => {
        {
            let mut y = HashSet::new();
            y.insert($x);
            y
        }
    };
    [ $($x:expr),+ ] => {
        HashSet::from([ $($x),+ ])
    };
}
