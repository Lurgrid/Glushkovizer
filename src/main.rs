use glushkovizer::automata::{Automata, FinitAutomata};
use glushkovizer::regexp::RegExp;
fn main() {
    let a = RegExp::try_from("(a+b).(a*.b)");
    if let Err(s) = a {
        eprintln!("Error ! {}", s);
        return;
    }
    let a = a.unwrap();
    println!("{:?}", a);
    let g = FinitAutomata::from(a);
    println!("{}", g);
    println!(
        "L'automate reconnais le mot ?: {}",
        g.accept(&("ab".chars().collect::<Vec<char>>()[..]))
    );
    let mut g2: FinitAutomata<char> = FinitAutomata::new();
    g2.add_state();
    g2.add_state();
    g2.add_initial(0);
    g2.add_final(1);
    g2.add_transition(0, 1, 'a');
    println!(
        "L'automate reconnais le mot ?: {}",
        g2.accept(&("a".chars().collect::<Vec<char>>()[..]))
    );
}
