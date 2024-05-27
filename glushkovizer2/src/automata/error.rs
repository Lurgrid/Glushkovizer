//! Module designed to facilitate the management of errors that may occur
//! during automata operation

#[derive(thiserror::Error, Debug)]
/// Enumeration of possible errors when manipulating automata
pub enum AutomataError {
    #[error("Unknow state 'from' given")]
    /// Error representing the fact that the state to be modified is unknown
    UnknowStateFrom,
    #[error("Unknow state 'to' given")]
    /// Error representing the fact that the arrival state does not exist
    UnknowStateTo,
    #[error("Unknow state given")]
    /// Error representing the fact that the searched state does not exist
    UnknowState,
    #[error("Not enough state given")]
    /// Error representing the fact that not enough state given in
    /// parameter
    NotEnoughState,
    #[error("Duplicate state")]
    /// Error representing the fact that a state has a duplicate
    DuplicateState,
}

/// Result alias type for automaton manipulation functions
pub type Result<T> = core::result::Result<T, AutomataError>;
