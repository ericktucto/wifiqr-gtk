use gio::glib::subclass::InitializingObject;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, Button, Label, glib, gio};
use once_cell::sync::Lazy;
use uuid::Uuid;
use std::cell::RefCell;
use std::rc::Rc;
use glib::value::ToValue;

use crate::wifi::Wifi;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ericktucto/wifiqr/row.ui")]
pub struct RowImpl {
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub label: TemplateChild<Label>,
    pub codigo: Rc<RefCell<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for RowImpl {
    const NAME: &'static str = "Row";
    type Type = Row;
    type ParentType = gtk::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RowImpl {
    fn constructed(&self) {
        self.parent_constructed();
    }
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::builder("codigo").build(),
            ]
        });
        PROPERTIES.as_ref()
    }
    fn set_property(&self, _id: usize, _value: &glib::Value, _pspec: &glib::ParamSpec) {
        match _pspec.name() {
            "codigo" => {
                let input_value = _value.to_owned().get().expect("Debe ser un 'String'");
                self.codigo.replace(input_value);
            }
            _ => unimplemented!()
        }
    }

    fn property(&self, _id: usize, _pspec: &glib::ParamSpec) -> glib::Value {
        match _pspec.name() {
            "codigo" => self.codigo.borrow().to_value(),
            _ => unimplemented!()
        }
    }

}
impl WidgetImpl for RowImpl {}
// Trait shared by Container
impl ContainerImpl for RowImpl {}
// Trait shared by Bindable widgets
impl BinImpl for RowImpl {}
impl ListBoxRowImpl for RowImpl {}
glib::wrapper! {
    pub struct Row(ObjectSubclass<RowImpl>)
        @extends gtk::Widget, gtk::ListBoxRow, gtk::AboutDialog,
        @implements gtk::Buildable, gtk::Actionable, gio::ActionGroup, gio::ActionMap,
                    gtk::ActionBar;
}

impl Row {
    pub fn new(wifi: &Wifi) -> Self {
        let codigo = Self::create_image(wifi);
        let obj: Row = glib::Object::builder()
            .property("codigo", codigo)
            .build();
        obj.set_label(wifi.get_name().as_str());
        obj
    }

    pub fn set_label(&self, label: &str) {
        self.imp().label.set_label(label);
    }

    fn create_image(w: &Wifi) -> String {
        let nombre = Uuid::new_v4().to_string() + &String::from(".png");
        let nombre = String::from("/tmp/wifiqr/") + &nombre;
        w.image().save(nombre.clone()).unwrap();
        nombre
    }

    pub fn codigo(&self) -> String {
        let codigo = self.imp().codigo.borrow().to_value();
        codigo.get::<String>().unwrap()
    }

    pub fn connect_qrcode<F: Fn(&gtk::Button) + 'static>(&self, funcion: F) {
        self.imp().button.connect_clicked(funcion);
    }
}
