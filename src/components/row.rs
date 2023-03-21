use gio::glib::subclass::InitializingObject;
use gtk4 as gtk;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, Button, Label, glib};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;

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
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RowImpl {
    fn constructed(&self) {
        self.parent_constructed();
        self.button.connect_clicked(glib::clone!(@weak self as ctx => move |_| {
            println!("{:?}", ctx.codigo.borrow());
        }));
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
impl ListBoxRowImpl for RowImpl {}
glib::wrapper! {
    pub struct Row(ObjectSubclass<RowImpl>)
        @extends gtk::Widget, gtk::ListBoxRow, gtk::AboutDialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Actionable, gio::ActionGroup, gio::ActionMap,
                    gtk::ActionBar, gtk::ATContext;
}

impl Row {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
