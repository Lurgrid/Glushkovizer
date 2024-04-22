use glushkovizer::regexp::RegExp;

fn main() {
    let a = RegExp::try_from("(a+b)*.a.$");
    match a {
        Ok(r) => println!("{:?}", r),
        Err(s) => eprintln!("Error: {}", s),
    }
}
