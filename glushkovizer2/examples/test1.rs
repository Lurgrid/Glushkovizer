use glushkovizer::automata::Automata;
use glushkovizer::prelude::*;

#[allow(unused_must_use)]

fn main() {
    let mut a: Automata<usize, char> = Automata::new();
    dbg!(a.add_state('a'));
    dbg!(a.add_initial(&'a'));
    dbg!(a.add_final(&'b'));
    dbg!(a.add_state('b'));
    dbg!(a.get_follow_count(&'a', &2usize));
    dbg!(&a);
    dbg!(a.remove_state(&'a'));
    dbg!(&a);
    dbg!(a.add_transition(&'b', &'b', 1usize));
    dbg!(a.add_transition(&'b', &'b', 1usize));
    dbg!(a.remove_transition(&'b', &'b', &1usize));
    dbg!(&a);
    dbg!(a.cloned());
}
