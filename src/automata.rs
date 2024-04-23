use dot::{Edges, GraphWalk, Id, LabelText, Labeller, Nodes};
use std::borrow::Cow;
use std::fmt::Debug;

use crate::regexp::RegExp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Numbered(char, usize);

#[derive(Debug)]
struct State {
    value: usize,
    transition: Vec<Numbered>,
}

#[derive(Debug)]
pub struct GlushAutomata {
    states: Vec<State>,
    finals: Vec<usize>,
}

#[derive(Debug)]
struct GlushInfo {
    firsts: Vec<Numbered>,
    lasts: Vec<Numbered>,
    null: bool,
    follows: Vec<(Numbered, Vec<Numbered>)>,
}

fn numbered(reg: RegExp<char>, start: usize) -> (RegExp<Numbered>, usize) {
    match reg {
        RegExp::Concat(l, r) => {
            let (nl, end) = numbered(*l, start);
            let (nr, end) = numbered(*r, end);
            (RegExp::Concat(Box::new(nl), Box::new(nr)), end)
        }
        RegExp::Or(l, r) => {
            let (nl, end) = numbered(*l, start);
            let (nr, end) = numbered(*r, end);
            (RegExp::Or(Box::new(nl), Box::new(nr)), end)
        }
        RegExp::Epsilon => (RegExp::Epsilon, start),
        RegExp::Symbol(v) => {
            let r = RegExp::Symbol(Numbered(v, start));
            let end = start + 1;
            (r, end)
        }
        RegExp::Times(c) => {
            let (nc, end) = numbered(*c, start);
            (RegExp::Times(Box::new(nc)), end)
        }
    }
}

fn get_glush_info(r: RegExp<Numbered>) -> GlushInfo {
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
impl From<RegExp<char>> for GlushAutomata {
    fn from(reg: RegExp<char>) -> Self {
        let (reg, end) = numbered(reg, 1);
        let mut info = get_glush_info(reg);
        let mut g = GlushAutomata {
            states: vec![],
            finals: vec![],
        };
        for i in 0..end {
            g.states.push(State {
                value: i,
                transition: vec![],
            });
        }
        for i in 1..end {
            let s = &mut g.states[i as usize];
            if let Some(&(_, ref l)) = info.follows.iter().find(|&&(ref s, _)| s.1 == i) {
                for next in l.iter() {
                    s.transition.push(*next);
                }
            }
        }
        g.finals = info.lasts.iter().map(|n| n.1).collect();
        g.states[0].transition.append(&mut info.firsts);
        g
    }
}

impl<'a> Labeller<'a, usize, (usize, usize, char)> for GlushAutomata {
    fn graph_id(&self) -> dot::Id {
        Id::new("glushkovs_automata").unwrap()
    }

    fn node_id(&self, n: &usize) -> dot::Id {
        Id::new(format!("N{}", *n)).unwrap()
    }

    fn node_label<'b>(&'b self, n: &usize) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(n.to_string().into())
    }

    fn edge_label<'b>(&'b self, e: &(usize, usize, char)) -> dot::LabelText<'b> {
        LabelText::LabelStr(format!("{}{}", e.2, e.1).into())
    }
}

impl<'a> GraphWalk<'a, usize, (usize, usize, char)> for GlushAutomata {
    fn nodes(&self) -> Nodes<'a, usize> {
        Cow::Owned(self.states.iter().map(|s| s.value).collect())
    }

    fn edges(&self) -> Edges<'a, (usize, usize, char)> {
        let mut edges: Vec<(usize, usize, char)> = vec![];
        for state in self.states.iter() {
            for neig in state.transition.iter() {
                edges.push((state.value, neig.1, neig.0));
            }
        }
        Cow::Owned(edges)
    }

    fn source(&self, e: &(usize, usize, char)) -> usize {
        e.0
    }

    fn target(&self, e: &(usize, usize, char)) -> usize {
        e.1
    }
}
