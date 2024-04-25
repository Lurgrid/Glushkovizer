use glushkovizer::regexp::RegExp;
use rand::Rng;
use std::usize;

const MAX_DEPTH: usize = 7;
const NB_REGEX: usize = 10_000;

#[test]
fn regex() {
    for i in 0..NB_REGEX {
        let r = gen_regex(MAX_DEPTH);
        let r2 = RegExp::try_from(r.to_string().as_str());
        if let Err(s) = r2 {
            panic!("Error on {}:\n{}\n{}", i, s, r.to_string());
        }
        let r2 = r2.unwrap();
        assert_eq!(r, r2);
    }
    println!("Success on {} tests !", NB_REGEX);
}

/// Renvoie un arbre représentant une expression régulière de hauteur maximal
/// "d".
fn gen_regex(d: usize) -> RegExp<char> {
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
            0..=1 => RegExp::Times(Box::new(gen_regex(d - 1))),
            2..=7 => RegExp::Or(Box::new(gen_regex(d - 1)), Box::new(gen_regex(d - 1))),
            _ => RegExp::Concat(Box::new(gen_regex(d - 1)), Box::new(gen_regex(d - 1))),
        },
    }
}
