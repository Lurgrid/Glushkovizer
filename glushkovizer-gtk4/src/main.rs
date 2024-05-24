mod glushkovizerapp;
use glushkovizerapp::{App, GlushkovizerApp};
use gtk::{glib, prelude::*};

const APP_ID: &str = "com.sagbot.GlushkovizerApp";

fn main() -> glib::ExitCode {
    let app = App::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &App) {
    let glush = GlushkovizerApp::new(app);
    glush.present();
}
