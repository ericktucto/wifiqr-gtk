use std::{ fmt::Debug, process::{Command, ChildStdout}, io::Read };
use image::{ ImageBuffer, Luma };
use qrcode::QrCode;
use regex::Regex;

use crate::commands::exec;

pub mod network_manager;
pub mod connman;

struct Attribute(String, String);

struct Seccion {
    name: String,
    attributes: Vec<Attribute>,
}

impl Seccion {
    fn make(name: &str, content: &str) -> Self {
        let re: Regex = Regex::new(r"(?P<nombre>.*)=(?P<valor>.*)\n").unwrap();
        let mut attributes: Vec<Attribute> = Vec::new();
        for cap in re.captures_iter(content) {
            attributes.push(Attribute(cap["nombre"].to_string(), cap["valor"].to_string()));
        }
        Self {
            name: name.to_string(),
            attributes,
        }
    }

    fn get_attribute(&self, name: &str) -> String {
        for attr in self.attributes.iter() {
            let Attribute(key, value) = attr;
            if key == name {
                return value.clone();
            }
        }
        return String::from("");
    }
}


fn show_content(path: String, password: Option<ChildStdout>) -> String {
    let args = vec!["-S", "-p", "''", "cat", path.as_str()];
    let resultado = exec("sudo".to_owned(), args, password);
    let mut output = String::new();
    resultado.stdout.unwrap().read_to_string(&mut output).unwrap();

    output
}

pub trait Lister {
    fn list(&self) -> Vec<Wifi>;
}
pub enum AdminNetwork {
    NetworkManager,
    ConnMan,
    NoKnown,
}
pub fn get_name_admin_network() -> AdminNetwork {
    let output = Command::new("ps")
        .arg("-eo")
        .arg("comm=")
        .output()
        .expect("failed to execute process");

    let process_list = String::from_utf8_lossy(&output.stdout);

    if process_list.contains("NetworkManager") {
        AdminNetwork::NetworkManager
    } else if process_list.contains("connmand") {
        AdminNetwork::ConnMan
    } else {
        AdminNetwork::NoKnown
    }
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
