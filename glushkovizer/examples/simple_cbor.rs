use glushkovizer::prelude::*;
use glushkovizer::{automata::Automata, regexp::RegExp};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let reg = RegExp::try_from("(a+b).a*.b*.(a+b)*")?;
    let auto = Automata::from(reg);
    println!("1 - {}", auto.to_dot(true).unwrap());
    let s = serde_cbor::to_vec(&auto)?;
    println!("to cbor\n {:?}", s);
    let auto2: Automata<char, usize> = serde_cbor::from_slice(&s[..])?;
    println!("2 - {}", auto2.to_dot(true).unwrap());
    Ok(())
}
