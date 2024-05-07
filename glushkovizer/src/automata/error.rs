//! Module ayant pour but de facilité la gestion des erreurs qui peuvent arriver
//! lors de la manipulation des automates

#[derive(thiserror::Error, Debug)]
/// Enumeration des erreurs possible lors de la manipulation des automates
pub enum AutomataError {
    #[error("Unknow state 'from' given")]
    /// Erreur representant le fait que l'état qu'on veut modifer est inconnu
    UnknowStateFrom,
    #[error("Unknow state 'to' given")]
    /// Erreur repsesentant le fait que l'état d'arrivé est inconnu
    UnknowStateTo,
    #[error("Unknow state given")]
    /// Erreur representant le fait que l'état chercher n'existe pas
    UnknowState,
    #[error("Not enough state given")]
    /// Erreur representant le fait qu'il n'y à pas assez d'état donné en
    /// parametre.
    NotEnoughState,
    #[error("Duplicate state")]
    /// Erreur representant le fait qu'un état à un doublon
    DuplicateState,
}

/// Type alias des resultat des fonction de manipulation des automates
pub type Result<T> = core::result::Result<T, AutomataError>;
