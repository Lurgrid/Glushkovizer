use glushkovizer::{automata::Automata, regexp::RegExp};
use rand::Rng;
use std::{array, usize};

const MAX_DEPTH: usize = 4;
const NB_WORD: usize = 100;
const NB_TEST: usize = 10;
const NB_REPEAT: usize = 10;

#[test]
fn automata() {
    for i in 0..NB_TEST {
        let r = gen_regex(MAX_DEPTH);
        let w: [String; NB_WORD] = gen_words(&r);
        let a = Automata::from(r);
        for word in w {
            if !a.accept(word.chars().collect::<Vec<char>>().iter()) {
                panic!("Error on {}:\n{}\n{}", i, word, a);
            }
        }
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

/// Renvoie un tableau de [String] de taille [NB], qui est reconnu par
/// l'expression régulière.
pub fn gen_words<const NB: usize>(reg: &RegExp<char>) -> [String; NB] {
    let mut rng = rand::thread_rng();
    match reg {
        &RegExp::Epsilon => array::from_fn(|_| String::new()),
        &RegExp::Symbol(s) => array::from_fn(|_| String::from(s)),
        &RegExp::Repeat(ref s) => {
            gen_words(s.as_ref()).map(|w| w.repeat(rng.gen_range(0..NB_REPEAT)))
        }
        &RegExp::Concat(ref l, ref r) => {
            let mut l = gen_words(l.as_ref());
            let r: [String; NB] = gen_words(r.as_ref());
            for (a, b) in l.iter_mut().zip(r) {
                *a += &b;
            }
            l
        }
        &RegExp::Or(ref l, ref r) => {
            let mut l = gen_words(l.as_ref());
            let r: [String; NB] = gen_words(r.as_ref());
            for (a, b) in l.iter_mut().zip(r) {
                if rng.gen_bool(0.5) {
                    *a = b;
                }
            }
            l
        }
    }
}
