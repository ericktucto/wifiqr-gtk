use gtk::prelude::*;
use gtk::glib::subclass::{ InitializingObject, Signal };
use gtk::subclass::prelude::*;
use gtk::{glib, gio, CompositeTemplate, Button, Entry, Label, Image};
use once_cell::sync::Lazy;

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
    #[template_child]
    pub message: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for PasswordImpl {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "ModalPassword";
    type Type = Password;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PasswordImpl {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        let image = Image::from_icon_name(
            Some("view-conceal-symbolic"),
            gtk::IconSize::Button
        );
        self.toggle.set_image(Some(&image));
        self.cancelar.connect_clicked(|_| {
            std::process::exit(0);
        });
        self.input.connect_activate(glib::clone!(@weak self as ctx => move |_| {
            ctx.authentication();
        }));
        self.aceptar.connect_clicked(glib::clone!(@weak self as ctx => move |_| {
            ctx.authentication();
        }));
        self.toggle.connect_clicked(glib::clone!(@weak self as ctx => move |button: &Button| {
            let is_visibility = ctx.input.property::<bool>("visibility");
            if is_visibility {
                ctx.input.set_visibility(false);
                let image = Image::from_icon_name(
                    Some("view-conceal-symbolic"),
                    gtk::IconSize::Button
                );
                button.set_image(Some(&image));
            } else {
                ctx.input.set_visibility(true);
                let image = Image::from_icon_name(
                    Some("view-reveal-symbolic"),
                    gtk::IconSize::Button
                );
                button.set_image(Some(&image));
            }
        }));
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
// Trait shared by Container
impl ContainerImpl for PasswordImpl {}
// Trait shared by Bindable widgets
impl BinImpl for PasswordImpl {}

// Trait shared by all windows
impl WindowImpl for PasswordImpl {}

// Trait shared by all application windows
impl ApplicationWindowImpl for PasswordImpl {}


impl PasswordImpl {
    fn authentication(&self) {
        let input = self.input.clone();
        if check_password(&input.text().as_str()) {
            // create list saved wifi
            let password: String = String::from(input.text().as_str());
            self.obj().emit_by_name::<()>("authorized", &[&password]);
        }
        self.message.set_visible(true);
    }

}

glib::wrapper! {
    pub struct Password(ObjectSubclass<PasswordImpl>)
        @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Buildable;
}

impl Password {
    pub fn new() -> Self {
        // Create new window
        glib::Object::builder()
            //.property("transient-for", "app")
            .build()
    }
}

