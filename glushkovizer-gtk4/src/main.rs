mod glushkovizerapp;
use glushkovizerapp::GlushkovizerApp;
use gtk::prelude::*;
use gtk::{gio, glib};

const APP_ID: &str = "com.sagbot.GlushkovizerApp";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("glushkovizer.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
fn build_ui(app: &adw::Application) {
    let glush = GlushkovizerApp::new(app);
    glush.present();
}
