use gtk4 as gtk;
use gtk::glib::subclass::{ InitializingObject, Signal };
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button, Entry};
use once_cell::sync::Lazy;

use super::Window;
use crate::helpers::check_password;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ericktucto/wifiqr/password.ui")]
pub struct PasswordImpl {
    #[template_child]
    pub aceptar: TemplateChild<Button>,
    #[template_child]
    pub cancelar: TemplateChild<Button>,
    #[template_child]
    pub toggle: TemplateChild<Button>,
    #[template_child]
    pub input: TemplateChild<Entry>,
}

#[glib::object_subclass]
impl ObjectSubclass for PasswordImpl {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "ModalPassword";
    type Type = Password;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PasswordImpl {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("authorized").param_types([String::static_type()]).build()]
        });
        SIGNALS.as_ref()
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for PasswordImpl {}

// Trait shared by all windows
impl WindowImpl for PasswordImpl {}

// Trait shared by all application windows
impl ApplicationWindowImpl for PasswordImpl {}

#[gtk::template_callbacks]
impl PasswordImpl {
    #[template_callback]
    fn handler_cancelar_clicked(&self, _: gtk::Button) {
        std::process::exit(0);
    }
    #[template_callback]
    fn handler_aceptar_clicked(&self, _: gtk::Button) {
        self.authentication();
    }
    #[template_callback]
    fn handler_input_activate(&self, _: gtk::Entry) {
        self.authentication();
    }
    #[template_callback]
    fn handler_toggle_clicked(&self, button: gtk::Button) {
        let is_visibility = self.input.property::<bool>("visibility");
        if is_visibility {
            self.input.set_visibility(false);
            button.set_icon_name("view-conceal-symbolic");
        } else {
            self.input.set_visibility(true);
            button.set_icon_name("view-reveal-symbolic");
        }
    }
}

impl PasswordImpl {
    fn authentication(&self) {
        let input = self.input.clone();
        if check_password(&input.text().as_str()) {
            // create list saved wifi
            let password: String = String::from(input.text().as_str());
            self.obj().emit_by_name::<()>("authorized", &[&password]);
        }
    }

}

glib::wrapper! {
    pub struct Password(ObjectSubclass<PasswordImpl>)
        @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Password {
    pub fn new(window: &Window) -> Self {
        // Create new window
        glib::Object::builder()
            .property("transient-for", window)
            .build()
    }
}

