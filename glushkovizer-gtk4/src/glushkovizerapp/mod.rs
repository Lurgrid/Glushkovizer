mod imp;

use glib::Object;
use gtk::gdk::Display;
use gtk::{gio, glib};
use gtk::{prelude::*, CssProvider};

#[cfg(feature = "no-adwaita")]
pub type App = gtk::Application;
#[cfg(not(feature = "no-adwaita"))]
pub type App = adw::Application;

glib::wrapper! {
    pub struct GlushkovizerApp(ObjectSubclass<imp::GlushkovizerApp>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GlushkovizerApp {
    pub fn new(app: &App) -> Self {
        gio::resources_register_include!("glushkovizer.gresource")
            .expect("Failed to register resources.");
        app.set_accels_for_action("win.save", &["<Ctrl>s"]);
        app.set_accels_for_action("win.open", &["<Ctrl>o"]);

        let provider = CssProvider::new();
        provider.load_from_resource("/com/sagbot/GlushkovApp/style.scss");
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        Object::builder().property("application", app).build()
    }
}
