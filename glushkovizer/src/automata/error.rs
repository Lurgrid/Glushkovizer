//! Module ayant pour but de facilité la gestion des erreurs qui peuvent arriver
//! lors de la manipulation des automates

use std::{error::Error, fmt::Display};

#[derive(Debug)]
/// Enumeration des erreurs possible lors de la manipulation des automates
pub enum AutomataError {
    /// Erreur representant le fait que l'état qu'on veut modifer est inconnu
    UnknowStateFrom,
    /// Erreur repsesentant le fait que l'état d'arrivé est inconnu
    UnknowStateTo,
    /// Erreur representant le fait que l'état chercher n'existe pas
    UnknowState,
    /// Erreur representant le fait qu'il n'y à pas assez d'état donné en
    /// parametre.
    NotEnoughState,
    /// Erreur representant le fait qu'un état à un doublon
    DuplicateState,
}

/// Type alias des resultat des fonction de manipulation des automates
pub type Result<T> = std::result::Result<T, AutomataError>;

impl Display for AutomataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AutomataError {}
