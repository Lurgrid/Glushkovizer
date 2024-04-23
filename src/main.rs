use dot::render;
use glushkovizer::automata::GlushAutomata;
use glushkovizer::regexp::RegExp;
use std::fs::File;

fn main() {
    let a = RegExp::try_from("(a+b)*.a.b+$");
    if let Err(s) = a {
        eprintln!("Error ! {}", s);
        return;
    }
    let a = a.unwrap();
    println!("{:?}", a);
    let g = GlushAutomata::from(a);
    println!("{:?}", g);
    let mut f = File::create("glushkov.dot").unwrap();
    render(&g, &mut f).unwrap();
}
