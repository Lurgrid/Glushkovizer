use glushkovizer::automata::Automata;
use glushkovizer::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let auto = Automata::new();
    auto.add_state(0);
    auto.add_state(1);
    auto.add_state(2);
    auto.add_initial(&0)?;
    auto.add_final(&2)?;
    auto.add_transition(&0, &1, 'a')?;
    auto.add_transition(&0, &2, 'a')?;
    auto.add_transition(&2, &1, 'b')?;
    auto.add_transition(&2, &0, 'b')?;
    auto.add_transition(&1, &0, 'a')?;
    println!("{}", auto.to_dot(false)?);
    let auto = auto.homogenized();
    println!("{}", auto.to_dot(false)?);
    Ok(())
}
