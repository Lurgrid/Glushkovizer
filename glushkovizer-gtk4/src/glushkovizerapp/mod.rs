mod imp;

use glib::Object;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct GlushkovizerApp(ObjectSubclass<imp::GlushkovizerApp>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GlushkovizerApp {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
}