use glushkovizer::{automata::Automata, regexp::RegExp};
use petgraph::algo::kosaraju_scc;
use rand::Rng;
use std::usize;

const MAX_DEPTH: usize = 7;
const NB_TEST: usize = 10_000;

#[test]
fn kosaraju() {
    for _ in 0..NB_TEST {
        let r = gen_regex(MAX_DEPTH);
        let a = Automata::from(r);
        let mut ka = a.kosaraju();
        let g = a.get_graph();
        let mut kp: Vec<Vec<usize>> = kosaraju_scc(&g)
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|i| *g.node_weight(i).unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();
        ka.iter_mut().for_each(|v| v.sort());
        ka.sort_by_key(|inner_vec| inner_vec[0]);
        kp.iter_mut().for_each(|v| v.sort());
        kp.sort_by_key(|inner_vec| inner_vec[0]);
        assert_eq!(ka, kp);
    }
}

/// Renvoie un arbre représentant une expression régulière de hauteur maximal
/// "d".
pub fn gen_regex(d: usize) -> RegExp<char> {
    let mut rng = rand::thread_rng();
    match d {
        0 => {
            if rng.gen_bool(0.5) {
                RegExp::Epsilon
            } else {
                RegExp::Symbol(rng.gen_range('a'..'z'))
            }
        }
        _ => match rng.gen_range(0..12) {
            0..=1 => RegExp::Repeat(Box::new(gen_regex(d - 1))),
            2..=7 => RegExp::Or(Box::new(gen_regex(d - 1)), Box::new(gen_regex(d - 1))),
            _ => RegExp::Concat(Box::new(gen_regex(d - 1)), Box::new(gen_regex(d - 1))),
        },
    }
}
