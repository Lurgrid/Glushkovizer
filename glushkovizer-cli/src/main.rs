use glushkovizer::automata::Automata;
use glushkovizer::regexp::RegExp;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::{stdin, Error, Result, Write};
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};

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
        m.pop().unwrap();

        let a = RegExp::try_from(m.as_str());
        if let Err(s) = a {
            eprintln!("Error ! {}", s);
            continue;
        }
        let a = a.unwrap();
        println!("{:?}", a);
        let g = Automata::from(a);
        let mut scc = g.extract_scc();
        scc.push(g);
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
                    println!("{}", scc.last().unwrap());
                    continue 'main;
                }
                Ok(_) => {
                    m.pop().unwrap();
                    for i in 0..(scc.len()) {
                        m.push_str(i.to_string().as_str());
                        let path = Path::new(m.as_str());
                        if path.exists() {
                            eprintln!("Error ! File already existing");
                            continue;
                        }
                        let svg = get_svg(&scc[i]);
                        if let Err(s) = svg {
                            eprintln!("Error ! {}", s);
                            return ExitCode::FAILURE;
                        }
                        let svg = svg.unwrap();
                        let f = File::create(m.trim_end());
                        if let Err(s) = f {
                            eprintln!("Error ! {}", s);
                            return ExitCode::FAILURE;
                        }
                        let mut f = f.unwrap();
                        if let Err(s) = f.write_all(svg.as_bytes()) {
                            eprintln!("Error ! {}", s);
                            return ExitCode::FAILURE;
                        }
                        drop(f);
                        println!("Saved !");
                        m.pop().unwrap();
                    }
                    continue 'main;
                }
            }
        }
    }
    ExitCode::SUCCESS
}

/// Renvoie la représentation de "g" en SVG en cas de succès, sinon en cas
/// d'erreur renvoie cette erreur.
fn get_svg<T, V>(g: &Automata<T, V>) -> Result<String>
where
    T: Eq + Hash + Display + Clone,
    V: Eq + Hash + Display + Clone,
{
    use std::io::ErrorKind;
    let mut c = Command::new("dot")
        .arg("-Tsvg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(ref mut inp) = c.stdin {
        inp.write_all(g.to_string().as_bytes())?;
    } else {
        return Err(Error::new(ErrorKind::Other, "No input"));
    }
    let output = c.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)
        .map_err(|_| Error::new(ErrorKind::Other, "Not a valid utf-8 output"))?)
}
