//! Sous module permettant la gestion d'automate de Glushkov, avec une
//! convertion de RegExp en automate de Glushkov

use crate::automata::{Automata, FinitAutomata};
use crate::regexp::{Numbered, RegExp};
use std::hash::Hash;

/// Structure regroupant toute les informations nécessaire à la création d'un
/// automate de Glushkov.
struct GlushInfo<T> {
    firsts: Vec<Numbered<T>>,
    lasts: Vec<Numbered<T>>,
    null: bool,
    follows: Vec<(Numbered<T>, Vec<Numbered<T>>)>,
}

/// Fonction qui permet de calculer les informations nécessaire à la création
///     d'un automate de Glushkov de l'expression regulière "r".
///     Renvoie les informations de cette expression.
fn get_glush_info<T: Clone + Copy + PartialEq>(r: RegExp<Numbered<T>>) -> GlushInfo<T> {
    match r {
        RegExp::Epsilon => GlushInfo {
            firsts: vec![],
            lasts: vec![],
            null: true,
            follows: vec![],
        },
        RegExp::Symbol(s) => GlushInfo {
            firsts: vec![s],
            lasts: vec![s],
            null: false,
            follows: vec![(s, vec![])],
        },
        RegExp::Times(c) => {
            let mut gi = get_glush_info(*c);
            gi.null = true;
            for last in gi.lasts.iter() {
                if let Some(f) = gi.follows.iter_mut().find(|f| f.0 == *last) {
                    f.1.append(&mut gi.firsts.clone());
                }
            }
            gi
        }
        RegExp::Or(l, r) => {
            let mut gil = get_glush_info(*l);
            let mut gir = get_glush_info(*r);
            gil.firsts.append(&mut gir.firsts);
            gil.lasts.append(&mut gir.lasts);
            gil.null = gil.null || gir.null;
            gil.follows.append(&mut gir.follows);
            gil
        }
        RegExp::Concat(l, r) => {
            let mut gil = get_glush_info(*l);
            let mut gir = get_glush_info(*r);
            for last in gil.lasts.iter() {
                if let Some(f) = gil.follows.iter_mut().find(|f| f.0 == *last) {
                    f.1.append(&mut gir.firsts.clone());
                }
            }
            gil.follows.append(&mut gir.follows);
            if gil.null {
                gil.firsts.append(&mut gir.firsts);
            }
            if gir.null {
                gil.lasts.append(&mut gir.lasts);
            } else {
                gil.lasts = gir.lasts;
            }
            gil.null = gil.null && gir.null;
            gil
        }
    }
}
impl<T: Clone + Copy + PartialEq + Eq + Hash> From<RegExp<T>> for FinitAutomata<T> {
    fn from(reg: RegExp<T>) -> Self {
        let (reg, end) = reg.numbered(1);
        let info = get_glush_info(reg);
        let mut g = FinitAutomata::new();
        for _ in 0..end {
            g.add_state();
        }
        for i in 1..end {
            if let Some(&(_, ref l)) = info.follows.iter().find(|&&(ref s, _)| s.1 == i) {
                for next in l {
                    g.add_transition(i, next.1, next.0);
                }
            }
        }
        for f in info.lasts {
            g.add_final(f.1);
        }
        for i in info.firsts {
            g.add_transition(0, i.1, i.0);
        }
        g.add_initial(0);
        g
    }
}
