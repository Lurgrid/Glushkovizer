use glib::subclass::InitializingObject;
use glushkovizer::automata::Automata;
use glushkovizer::regexp::RegExp;
use gtk::gdk::Texture;
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, Align, Button, CompositeTemplate, Entry, Image, Label};
use gtk::{prelude::*, TextView};
use std::fmt::Display;
use std::hash::Hash;
use std::io::{Error, Result, Write};
use std::process::{Command, Stdio};

const IMAGE_MARG: i32 = 100;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/sagbot/GlushkovApp/glushkovizer.ui")]
pub struct GlushkovizerApp {
    #[template_child]
    pub content: TemplateChild<gtk::Box>,
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub list: TemplateChild<gtk::Box>,
    #[template_child]
    pub image: TemplateChild<Image>,
    #[template_child]
    pub error: TemplateChild<TextView>,
    #[template_child]
    pub orbit_title: TemplateChild<Label>,
    #[template_child]
    pub orbit: TemplateChild<gtk::Box>,
    startup: bool,
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

        self.orbit
            .bind_property("visible", &self.orbit_title.get(), "visible")
            .bidirectional()
            .sync_create()
            .build();
    }
}

#[template_callbacks]
impl GlushkovizerApp {
    #[template_callback]
    fn handle_parse_clicked(&self, _: &Button) {
        self.update();
    }

    #[template_callback]
    fn handle_entry_activate(&self, _: &Entry) {
        self.update();
    }
}

impl GlushkovizerApp {
    fn update(&self) {
        while let Some(child) = self.orbit.first_child() {
            self.orbit.remove(&child);
        }
        if !self.startup {
            self.list
                .bind_property("visible", &self.error.get(), "visible")
                .invert_boolean()
                .bidirectional()
                .sync_create()
                .build();
        }
        let sr = self.entry.text().to_string();
        let r = match RegExp::try_from(sr) {
            Err(s) => {
                self.error.buffer().set_text(s.as_str());
                self.error.set_visible(true);
                return;
            }
            Ok(r) => r,
        };
        let a = Automata::from(r);
        let width = self.obj().width() - IMAGE_MARG;
        let height = self.obj().height() - IMAGE_MARG;
        let texture = match get_automata_texture(&a, width, height) {
            Err(e) => {
                self.error.buffer().set_text(e.to_string().as_str());
                self.error.set_visible(true);
                return;
            }
            Ok(t) => t,
        };
        self.image.set_from_paintable(Some(&texture));
        self.image.set_size_request(width, height);
        self.list.set_visible(true);

        let scc = a
            .extract_scc()
            .into_iter()
            .filter(|a| a.is_maximal_orbit())
            .collect::<Vec<_>>();
        let nb: i32 = match scc.len() as i32 {
            0 => {
                self.orbit.set_visible(false);
                return;
            }
            len => {
                self.orbit.set_visible(true);
                len
            }
        };

        let owidth = width / nb;
        let oheight = height / nb;

        for automata in scc {
            let texture = match get_automata_texture(&automata, owidth, oheight) {
                Err(e) => {
                    self.error.buffer().set_text(e.to_string().as_str());
                    self.error.set_visible(true);
                    return;
                }
                Ok(t) => t,
            };
            let image = Image::from_paintable(Some(&texture));
            image.set_halign(Align::Fill);
            image.set_valign(Align::Fill);
            image.set_hexpand(true);
            image.set_vexpand(true);
            image.set_size_request(owidth, oheight);
            self.orbit.append(&image);
        }
    }
}

/// Renvoie une Texture représentant le graph, en cas d'erreur renvoie cette
/// erreur
fn get_automata_texture<T, V>(a: &Automata<T, V>, width: i32, height: i32) -> Result<Texture>
where
    T: Eq + Hash + Display + Clone,
    V: Eq + Hash + Display + Clone,
{
    let svg = get_svg(
        &a,
        gtk::Settings::default()
            .map(|s| s.property("gtk-application-prefer-dark-theme"))
            .unwrap_or(false),
    )?;
    let loader = PixbufLoader::new();

    loader.set_size(width, height);
    loader
        .write(svg.as_bytes())
        .expect("Cannot write on the PixbufLoader");
    loader.close().expect("Cannot close the PixbufLoader");
    let pixbuf = loader
        .pixbuf()
        .expect("Cannot convert the PixbufLoader to Pixbuf");
    Ok(Texture::for_pixbuf(&pixbuf))
}

/// Renvoie la représentation de "g" en SVG en cas de succès, sinon en cas
/// d'erreur renvoie cette erreur.
fn get_svg<T, V>(g: &Automata<T, V>, inverse: bool) -> Result<String>
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
        inp.write_all(g.to_dot(inverse).as_bytes())?;
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
