mod custom_button;
mod window;

use gio::{prelude::*, SimpleAction};
use glib::clone;
use gtk::{prelude::*, Application};
use window::Window;

const APP_ID: &str = "com.man.petrovich";

fn main() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.set_accels_for_action("win.close", &["<Ctrl>Q"]);
    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);

    let action_close = SimpleAction::new("close", None);
    action_close.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));
    window.add_action(&action_close);

    window.present();
}
