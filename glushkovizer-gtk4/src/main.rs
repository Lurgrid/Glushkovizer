use glushkovizer::{automata::Automata, regexp::RegExp};
use gtk::{gdk_pixbuf::PixbufLoader, glib, prelude::*};
use std::fmt::Display;
use std::hash::Hash;
use std::io::{Error, Result, Write};
use std::process::{Command, Stdio};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.builder_basics")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let ui_src = include_str!("./ui/glushkovizer.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window = builder
        .object::<gtk::Window>("glushkovizer")
        .expect("Couldn't get window");
    window.set_application(Some(application));
    let entry = builder
        .object::<gtk::Entry>("search")
        .expect("Couldn't get search entry");

    let image = builder
        .object::<gtk::Image>("image")
        .expect("Couln't get image");

    entry.connect_changed(move |entry: &gtk::Entry| {
        let r = RegExp::try_from(entry.text().to_string().as_str());
        if let Err(s) = r {
            println!("{}\n{}", s, entry.text());
            return;
        }
        let r = r.unwrap();
        let a = Automata::from(r);
        let loader = PixbufLoader::new();
        let svg = get_svg(&a).expect("Export to svg failed");
        loader.write(svg.as_bytes()).unwrap();
        loader.close().unwrap();
        let pixbuf = loader.pixbuf().unwrap();
        image.set_from_pixbuf(Some(&pixbuf));
    });

    window.present();
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
