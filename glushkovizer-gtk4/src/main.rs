mod glushkovizerapp;
use gdk::Display;
use glushkovizerapp::GlushkovizerApp;
use gtk::{gdk, prelude::*};
use gtk::{gio, glib, CssProvider};

const APP_ID: &str = "com.sagbot.GlushkovizerApp";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("glushkovizer.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("/com/sagbot/GlushkovApp/style.scss");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    let glush = GlushkovizerApp::new(app);
    glush.present();
}
