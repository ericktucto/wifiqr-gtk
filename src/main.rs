mod components;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ Application, gio };
//use window::Window;
use components::Window;

const APP_ID: &str = "com.ericktucto.wifiqr";

fn main() {
    // Register and include resources
    gio::resources_register_include!("app.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "starup" signal of `app`
    app.connect_startup(|_| {
        load_css();
    });
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
// ANCHOR_END: build_ui

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/com/ericktucto/wifiqr/style.css");

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}


/*
mod commands;
mod wifi;
mod helpers;

use std::{ process::Child,path::Path };

use gtk4 as gtk;
use gtk::{prelude::*, Widget, ApplicationWindow, ListBox, ScrolledWindow, Image, Window, Entry, Button };
//use glib::ExitCode;
use libadwaita::{ prelude::*, Application, ExpanderRow };
use uuid::Uuid;
use wifi::AdminNetwork;
use wifi::get_name_admin_network;

use crate::commands::exec;
use crate::commands::read_output;
use crate::wifi::{ Wifi, network_manager::NetworkManager, Lister, connman::ConnMan };
use crate::helpers::parent_window;


fn get_password(password: &str) -> Child {
    let args: Vec<&str> = vec![password];
    exec("echo".to_owned(), args, None)
}

fn check_password(password: &str) -> bool {
    let child = get_password(&password);
    let args = vec!["-S", "-p", "''", "whoami"];
    let result = exec("sudo".to_owned(), args, child.stdout);
    let l = read_output(result);
    l.trim().eq("root")
}

fn modal_password() -> gtk::Builder {
    let builder = gtk::Builder::from_file("/usr/share/wifiqr/password.ui");
    //let builder = gtk::Builder::from_file("/com/ericktucto/wifiqr/password.ui");
    let input: Entry = builder.object("input").unwrap();
    let cancelar: Button = builder.object("cancel").unwrap();
    let toggle: Button = builder.object("toggle").unwrap();

    toggle.connect_clicked(move |btn| {
        let is_visibility = input.property::<bool>("visibility");
        if is_visibility {
            input.set_visibility(false);
            btn.set_icon_name("view-conceal-symbolic");
        } else {
            input.set_visibility(true);
            btn.set_icon_name("view-reveal-symbolic");
        }
    });
    cancelar.connect_clicked(|_| {
        std::process::exit(0);
    });

    builder
}

fn create_image(w: &Wifi) -> Image {
    let nombre = Uuid::new_v4().to_string() + &String::from(".png");
    let nombre = String::from("/tmp/") + &nombre;
    w.image().save(nombre.clone()).unwrap();
    let ruta = Path::new(nombre.as_str());
    let image = Image::from_file(ruta);
    image.set_size_request(300, 300);
    image
}

fn create_expanders_wifi(password: String) -> ListBox {
    let lista = ListBox::new();
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

    for w in resultado.iter() {
        let row1 = ExpanderRow::new();
        row1.set_property("title", w.get_name());

        // image qr
        let image = create_image(&w);

        // add to ExpanderRow
        row1.add_row(&image);
        lista.insert(&row1, -1);
    }
    lista
}

fn main() {
    let app = Application::builder()
        .application_id("com.ericktucto.wifiqr")
        .build();

    app.connect_activate(|app| {
        // scrollwindow
        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

        // We create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Mi conexiones guardadas")
            .width_request(600)
            .height_request(400)
            .child(&scrolled_window)
            .build();

        let builder = modal_password();
        let modal: Window = builder.object("ventana").unwrap();
        modal.set_transient_for(Some(&window));
        // Show the window.
        window.connect_show(move |_| {
            modal.present();
        });

        let aceptar: Button = builder.object("aceptar").unwrap();
        let input: Entry = builder.object("input").unwrap();
        aceptar.connect_clicked(move |btn| {
            if check_password(&input.text().as_str()) {
                // create list saved wifi
                let password: String = String::from(input.text().as_str());
                let lista = create_expanders_wifi(password);
                scrolled_window.set_child(Some(&lista));
                let modal = parent_window(btn.clone().upcast::<Widget>());
                modal.hide();
            }
        });

        window.show();
    });

    app.run();
}
*/
