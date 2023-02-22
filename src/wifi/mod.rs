use std::fmt::Debug;
use image::{ ImageBuffer, Luma };
use qrcode::QrCode;

pub mod network_manager;

pub trait Lister {
    fn list(&self) -> Vec<Wifi>;
}

#[derive(Debug)]
pub struct Wifi {
    name: String,
    password: String,
}

impl Wifi {
    fn new() -> Self {
        Self {
            name: String::from(""),
            password: String::from(""),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn image(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let wifistring = String::from("WIFI:T:WPA;S:{ssid};P:{password};;");
        let wifistring = wifistring.replace("{ssid}", self.name.as_str());
        let wifistring = wifistring.replace("{password}", self.password.as_str());
        let code = QrCode::new(wifistring.as_bytes()).unwrap();
        return code.render::<Luma<u8>>().build();
    }
}
