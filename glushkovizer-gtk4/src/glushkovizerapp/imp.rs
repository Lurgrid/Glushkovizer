use glib::subclass::InitializingObject;
use glushkovizer::automata::Automata;
use glushkovizer::regexp::RegExp;
use gtk::gdk::Texture;
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, Button, CompositeTemplate, Entry, Image};
use gtk::{prelude::*, TextView};
use std::fmt::Display;
use std::hash::Hash;
use std::io::{Error, Result, Write};
use std::process::{Command, Stdio};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/sagbot/GlushkovApp/glushkovizer.ui")]
pub struct GlushkovizerApp {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub list: TemplateChild<gtk::Box>,
    #[template_child]
    pub image: TemplateChild<Image>,
    #[template_child]
    pub error: TemplateChild<TextView>,
}

#[glib::object_subclass]
impl ObjectSubclass for GlushkovizerApp {
    const NAME: &'static str = "GlushkovizerApp";
    type Type = super::GlushkovizerApp;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GlushkovizerApp {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

#[template_callbacks]
impl GlushkovizerApp {
    #[template_callback]
    fn handle_parse_clicked(&self, _: &Button) {
        self.change_image();
    }

    #[template_callback]
    fn handle_entry_activate(&self, _: &Entry) {
        self.change_image();
    }
}

impl GlushkovizerApp {
    fn change_image(&self) {
        let sr = self.entry.text().to_string();
        let r = RegExp::try_from(sr);
        if let Err(s) = r {
            self.list.set_visible(false);
            self.error.set_visible(true);
            self.error.buffer().set_text(s.as_str());
            return;
        }
        let r = r.unwrap();
        let a = Automata::from(r);
        let svg = get_svg(&a);
        if let Err(s) = svg {
            self.list.set_visible(false);
            self.error.set_visible(true);
            self.error.buffer().set_text(s.to_string().as_str());
            return;
        }
        self.error.set_visible(false);
        let svg = svg.unwrap();
        let loader = PixbufLoader::new();

        loader.set_size(self.obj().width(), self.obj().height());
        loader.write(svg.as_bytes()).unwrap();
        loader.close().unwrap();
        let pixbuf = loader.pixbuf().unwrap();
        let texture = Texture::for_pixbuf(&pixbuf);
        self.list.set_visible(true);
        self.image.set_from_paintable(Some(&texture));
    }
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

impl WidgetImpl for GlushkovizerApp {}

impl WindowImpl for GlushkovizerApp {}

impl ApplicationWindowImpl for GlushkovizerApp {}
