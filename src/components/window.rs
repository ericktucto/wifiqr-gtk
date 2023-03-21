use gtk4 as gtk;
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListBox};

use crate::components::Password;
use crate::components::row::Row;

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ericktucto/wifiqr/app.ui")]
pub struct MyWindowImpl {
    #[template_child]
    pub listbox: TemplateChild<ListBox>,
}
// ANCHOR_END: object

// ANCHOR: subclass
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MyWindowImpl {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "WifiQRWindow";
    type Type = Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
// ANCHOR_END: subclass

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for MyWindowImpl {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        let fila = Row::new();
        self.listbox.append(&fila);
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for MyWindowImpl {}

// Trait shared by all windows
impl WindowImpl for MyWindowImpl {}

// Trait shared by all application windows
impl ApplicationWindowImpl for MyWindowImpl {}

glib::wrapper! {
    pub struct Window(ObjectSubclass<MyWindowImpl>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        // Create new window
        let obj: Self = glib::Object::builder().property("application", app).build();
        let modal = Password::new(&obj);
        obj.connect_show(glib::clone!(@weak modal as ctx => move |_| {
            ctx.show();
        }));

        obj
    }
}

