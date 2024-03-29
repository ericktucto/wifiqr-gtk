use gtk4 as gtk;
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{ glib, CompositeTemplate, ListBox };
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::components::row::Row;
use crate::wifi::{ AdminNetwork, get_name_admin_network, Wifi, network_manager::NetworkManager, connman::ConnMan, Lister };

use super::{Password, ModalImage};

// ANCHOR: object
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ericktucto/wifiqr/app.ui")]
pub struct MyWindowImpl {
    #[template_child]
    pub listbox: TemplateChild<ListBox>,
    pub modalimage: Rc<RefCell<ModalImage>>,
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
        if Path::new("/tmp/wifiqr").exists() {
            std::fs::remove_dir_all("/tmp/wifiqr").expect("Directory not found");
        }
        std::fs::create_dir("/tmp/wifiqr").expect("Directory exists");
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
        let modal_password = Password::new(&obj);
        let modal_image = ModalImage::new(Some(&obj));
        obj.set_modalimage(modal_image);
        modal_password.connect(
            "authorized",
            false,
            |args| {
                let modal = args[0].get::<Password>().unwrap();
                let password = args[1].get::<String>().unwrap();
                if let Some(window) = modal.transient_for() {
                    let resultado: Vec<Wifi> = match get_name_admin_network() {
                        AdminNetwork::NetworkManager => {
                            let lister = NetworkManager { password };
                            lister.list()
                        },
                        AdminNetwork::ConnMan => {
                            let lister = ConnMan { password };
                            lister.list()
                        },
                        AdminNetwork::NoKnown => panic!("testing...")
                    };

                    let window = window.downcast::<Window>().expect("Error de conversion");
                    for w in resultado.iter() {
                        window.add_wifi(&w.clone());
                    }
                }
                modal.hide();
                None
            }
        );
        obj.connect_show(glib::clone!(@weak modal_password as pass => move |_| {
            pass.show();
        }));
        obj
    }

    fn set_modalimage(&self, modal: ModalImage) {
        self.imp().modalimage.replace(modal);
    }

    fn add_wifi(&self, wifi: &Wifi) {
        let fila = Row::new(wifi);
        let path = fila.codigo();
        let nombre = wifi.get_name();

        fila.connect_qrcode(glib::clone!(@weak self as ctx => move |_| {
            let modal = ctx.imp().modalimage.borrow();
            modal.set_title(Some(nombre.clone().as_str()));
            modal.set_image(path.clone());
            modal.show();
        }));
        self.imp().listbox.append(&fila);
    }
}

