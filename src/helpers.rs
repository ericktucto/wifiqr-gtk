use gtk4::prelude::*;
use gtk4::{Window, Widget};


pub fn parent_window(mut child: Widget) -> Window {
    loop {
        child = match child.parent() {
            Some(parent) => parent,
            _ => { break; }
        }
    }
    child.downcast::<Window>().expect("No es un Window")
}
