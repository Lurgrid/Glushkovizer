use glushkovizer::automata::inner_automata::{state::RefState, InnerAutomata};

fn main() {
    let mut a = InnerAutomata::new();

    let _s0 = RefState::new('a');
    let s0 = _s0.clone();
    a.add_state(_s0);
    s0.add_follow(s0.clone(), 'a');
}
