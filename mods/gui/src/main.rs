use gio::prelude::*;
use gtk::{prelude::*, Application, ApplicationWindow, Box, Button, Entry, Label};
use once_cell::sync::OnceCell;
use petrovich_core::{case::Case, gender::Gender, Petrovich};

static PETROVICH: OnceCell<Petrovich> = OnceCell::new();

fn main() {
    let pv = Petrovich::new("mods/core/petrovich-rules/rules.yml").unwrap();
    PETROVICH.set(pv).unwrap();

    let application = Application::new(Some("com.man.petrovich"), Default::default());

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Petrovich"));
        window.set_default_size(450, 300);

        let first_name = Entry::new();
        first_name.set_placeholder_text(Some("First name"));
        first_name.set_tooltip_text(Some("First name"));
        first_name.set_input_purpose(gtk::InputPurpose::Alpha);

        let last_name = Entry::new();
        last_name.set_placeholder_text(Some("Last name"));
        last_name.set_tooltip_text(Some("Last name"));
        last_name.set_input_purpose(gtk::InputPurpose::Alpha);

        let patronimic_name = Entry::new();
        patronimic_name.set_placeholder_text(Some("Patronimic name"));
        patronimic_name.set_tooltip_text(Some("Patronimic name"));
        patronimic_name.set_input_purpose(gtk::InputPurpose::Alpha);

        let out_label = Label::new(None);

        let button = Button::with_label("Find");

        let out = out_label.clone();
        let fnc = first_name.clone();
        let lnc = last_name.clone();
        let pnc = patronimic_name.clone();
        button.connect_clicked(move |_| {
            println!("Clicked!");

            let pv = PETROVICH.get().unwrap();
            let f = pv.first_name(&fnc.text(), &Gender::Male, &Case::Accusative);
            let l = pv.first_name(&lnc.text(), &Gender::Male, &Case::Accusative);
            let p = pv.first_name(&pnc.text(), &Gender::Male, &Case::Accusative);
            let mut res = String::new();
            if !f.is_empty() {
                res += &f;
            }
            if !l.is_empty() {
                if !res.is_empty() {
                    res += " ";
                }
                res += &l;
            }
            if !p.is_empty() {
                if !res.is_empty() {
                    res += " ";
                }
                res += &p;
            }
            out.set_text(&res);
        });

        let vbox = Box::new(gtk::Orientation::Vertical, 2);
        vbox.append(&first_name);
        vbox.append(&last_name);
        vbox.append(&patronimic_name);
        vbox.append(&button);
        vbox.append(&out_label);

        window.set_child(Some(&vbox));

        window.present();
    });

    application.run();
}
