use gtk::glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, gio, CompositeTemplate};
use super::Window;


#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ericktucto/wifiqr/image.ui")]
pub struct ImageImpl {
    #[template_child]
    pub image: TemplateChild<gtk::Image>,
    #[template_child]
    pub aceptar: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for ImageImpl {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "ModalImage";
    type Type = ModalImage;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
impl ObjectImpl for ImageImpl {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        self.aceptar.connect_clicked(glib::clone!(@weak self as ctx => move|_| {
            ctx.obj().close();
        }));
    }
}
// ANCHOR_END: object_impl

// Trait shared by all widgets
impl WidgetImpl for ImageImpl {}
// Trait shared by Container
impl ContainerImpl for ImageImpl {}
// Trait shared by Bindable widgets
impl BinImpl for ImageImpl {}

// Trait shared by all windows
impl WindowImpl for ImageImpl {}

// Trait shared by all application windows
impl ApplicationWindowImpl for ImageImpl {}

glib::wrapper! {
    pub struct ModalImage(ObjectSubclass<ImageImpl>)
        @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Buildable;
}

impl ModalImage {
    pub fn new(window: Option<&Window>) -> Self {
        // Create new window
        glib::Object::builder()
            .property("transient-for", window)
            .build()
    }

    pub fn set_image(&self, file: String) {
        self.imp().image.set_file(Some(file.as_str()));
    }
}

impl Default for ModalImage {
    fn default() -> Self {
        Self::new(None)
    }
}
