use glushkovizer::prelude::*;
use glushkovizer::{
    automata::{Automata, SubAutomata},
    regexp::RegExp,
};
use std::{
    fmt::Display,
    hash::Hash,
    io::{stdin, Error, Result, Write},
    process::{Command, ExitCode, ExitStatus, Stdio},
};

fn main() -> ExitCode {
    let mut m = String::new();
    'main: loop {
        println!("Please enter a regular expression - Press Ctrl + D to quit");
        m.clear();
        match stdin().read_line(&mut m) {
            Err(s) => {
                eprintln!("Error ! {}", s);
                return ExitCode::FAILURE;
            }
            Ok(0) => break,
            Ok(_) => {}
        }
        let mt = m.trim();

        let a = RegExp::try_from(mt);
        if let Err(s) = a {
            eprintln!("Error ! {}", s);
            continue;
        }
        let a = a.unwrap();
        println!("{:?}", a);
        let g = Automata::from(a);
        let scc: Vec<SubAutomata<char, usize>> = g
            .extract_scc()
            .into_iter()
            .filter(|a| a.is_orbit())
            .collect();
        loop {
            println!(
                "Enter a filename to save the automata - Press Ctrl + D to \
                not save"
            );
            m.clear();
            match stdin().read_line(&mut m) {
                Err(s) => {
                    eprintln!("Error ! {}", s);
                    return ExitCode::FAILURE;
                }
                Ok(0) => {
                    println!("{}", g.to_dot(false).unwrap());
                    continue 'main;
                }
                Ok(_) => {
                    let m = m.trim();
                    if let Err(r) = save_svg(&g, m) {
                        eprintln!("Error ! {}", r);
                        return ExitCode::FAILURE;
                    }
                    if let Err(r) = scc
                        .into_iter()
                        .enumerate()
                        .try_for_each(|(i, sub)| save_svg(&sub, &format!("{m}{i}")).map(|_| ()))
                    {
                        eprintln!("Error ! {}", r);
                        return ExitCode::FAILURE;
                    }
                    println!("Saved !");
                    continue 'main;
                }
            }
        }
    }
    ExitCode::SUCCESS
}

/// Renvoie la représentation de "g" en SVG en cas de succès, sinon en cas
/// d'erreur renvoie cette erreur.
fn save_svg<'a, T, V>(g: &impl ToDot<'a, T, V>, name: &str) -> Result<ExitStatus>
where
    T: Eq + Hash + Clone + Display,
    V: Eq + Clone + Display,
{
    use std::io::ErrorKind;
    let mut c = Command::new("dot")
        .args(["-Tsvg", "-o", name])
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(ref mut inp) = c.stdin {
        inp.write_all(g.to_dot(false).unwrap().as_bytes())?;
    } else {
        return Err(Error::new(ErrorKind::Other, "No input"));
    }
    c.wait()
}
