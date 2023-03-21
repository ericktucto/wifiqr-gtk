use gio::glib::subclass::InitializingObject;
use gtk4 as gtk;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, Button, Label, glib};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ericktucto/wifiqr/row.ui")]
pub struct RowImpl {
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub label: TemplateChild<Label>,
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
            println!("{:?}", ctx.label.text());
        }));
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
