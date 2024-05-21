use glushkovizer::{automata::Automata, regexp::RegExp};
use serde_json::json;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let reg = RegExp::try_from("(a+b).a*.b*.(a+b)*")?;
    let auto = Automata::from(reg);
    println!("1 - {}", auto);
    let s = serde_json::to_string(&auto)?;
    println!("to json\n {}", s);
    let json = json!({
        "states": [0,1,2,3,4,5,6],
        "initials":[0],
        "finals":[2,5,1,6,4,3],
        "follow":[
            {"a":[1],"b":[2]},
            {"b":[6,4],"a":[5,3]},
            {"b":[6,4],"a":[3,5]},
            {"a":[3,5],"b":[6,4]},
            {"b":[6,4],"a":[5]},
            {"b":[6],"a":[5]},
            {"b":[6],"a":[5]}
        ]
    })
    .to_string();
    let auto2: Automata<char, usize> = serde_json::from_str(&json)?;
    println!("2 - {}", auto2);
    Ok(())
}
