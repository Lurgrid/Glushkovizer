use glushkovizer::prelude::*;
use glushkovizer::{automata::Automata, regexp::RegExp};
use serde_json::json;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let reg = RegExp::try_from("(a+b).a*.b*.(a+b)*")?;
    let auto = Automata::from(reg);
    println!("1 - {}", auto.to_dot(true).unwrap());
    let s = serde_json::to_string(&auto)?;
    println!("to json\n {}", s);
    let json = json!({
        "states": [3,2,1,0,4,5,6],
        "inputs": [0],
        "outputs": [6,3,1,2,4,5],
        "follows": [
            [3,"a",5],
            [3,"a",3],
            [3,"b",4],
            [3,"b",6],
            [2,"a",3],
            [2,"a",5],
            [2,"b",6],
            [2,"b",4],
            [1,"b",6],
            [1,"b",4],
            [1,"a",5],
            [1,"a",3],
            [0,"b",2],
            [0,"a",1],
            [4,"a",5],
            [4,"b",4],
            [4,"b",6],
            [5,"b",6],
            [5,"a",5],
            [6,"a",5],
            [6,"b",6]
        ]
    })
    .to_string();
    let auto2: Automata<char, usize> = serde_json::from_str(&json)?;
    println!("2 - {}", auto2.to_dot(true).unwrap());
    Ok(())
}
