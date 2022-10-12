mod custom_button;
mod window;

use gio::prelude::*;
use gtk::{prelude::*, Application};
use once_cell::sync::OnceCell;
use petrovich_core::{Petrovich};
use window::Window;

static PETROVICH: OnceCell<Petrovich> = OnceCell::new();
const APP_ID: &str = "com.man.petrovich";

fn main() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    let pv = Petrovich::new("mods/core/petrovich-rules/rules.yml").unwrap();
    PETROVICH.set(pv).unwrap();

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
