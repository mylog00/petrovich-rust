use std::cell::Cell;

use glib::subclass::InitializingObject;
use gtk::{prelude::*};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::custom_button::CustomButton;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/man/petrovich/window.ui")]
pub struct Window {
    pub number: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // Register `CustomButton`
        CustomButton::ensure_type();

        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_button_clicked(&self, button: &CustomButton) {
        let number_increased = self.number.get() + 1;
        self.number.set(number_increased);
        button.set_label(&number_increased.to_string())
    }
}

impl ObjectImpl for Window {}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}

