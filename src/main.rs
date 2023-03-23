mod commands;
mod components;
mod wifi;
mod helpers;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ Application, gio };
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

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}

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

