use glib::subclass::InitializingObject;
use gtk::{prelude::*, Entry, Label, DropDown};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use once_cell::sync::OnceCell;
use petrovich_core::Petrovich;
use petrovich_core::case::Case;
use petrovich_core::gender::Gender;

use crate::custom_button::CustomButton;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/man/petrovich/window.ui")]
pub struct Window {
    #[template_child]
    pub first_name: TemplateChild<Entry>,
    #[template_child]
    pub last_name: TemplateChild<Entry>,
    #[template_child]
    pub patronimic_name: TemplateChild<Entry>,
    #[template_child]
    pub gender: TemplateChild<DropDown>,
    #[template_child]
    pub case: TemplateChild<DropDown>,
    #[template_child]
    pub content: TemplateChild<Label>,
    petrovich: OnceCell<Petrovich>,
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
    fn handle_button_clicked(&self) {
        let pv = self.petrovich.get().unwrap();

        let gender = self.gender.selected();
        let gender = match gender {
            0 => Gender::Male,
            1 => Gender::Female,
            2 => Gender::Androgynous,
            _ => unreachable!()
        };
        let case = self.case.selected();
        let case = match case {
            0 => Case::Nominative,
            1 => Case::Genitive,
            2 => Case::Dative,
            3 => Case::Accusative,
            4 => Case::Instrumental,
            5 => Case::Prepositional,
            _ => unreachable!(),
        };

        let first_name = pv.first_name(self.first_name.text().as_str(), &gender, &case);
        let last_name = pv.first_name(self.last_name.text().as_str(), &gender, &case);
        let patronimic_name = pv.first_name(self.patronimic_name.text().as_str(), &gender, &case);

        let res = vec![last_name, first_name, patronimic_name].join(" ");
        self.content.set_text(&res);
    }
}

impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let pv = Petrovich::new("mods/core/petrovich-rules/rules.yml").expect("Loading petrovich rules");
        self.petrovich.set(pv).expect("Setting up petrovich object");
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}

