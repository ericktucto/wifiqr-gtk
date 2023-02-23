use std::{ process::Child, io::Read };
use regex::Regex;

use crate::commands::exec;

use super::{Wifi, Lister, show_content, Seccion};

const DIR_NETWORK: &str = "/etc/NetworkManager/system-connections/";

pub struct NetworkManager {
    pub password: String
}

impl NetworkManager {
    fn get_password(&self) -> Child {
        let args: Vec<&str> = vec![self.password.as_str()];
        exec("echo".to_owned(), args, None)
    }

    fn filter_networks(&self, networks: Vec<&str>) -> Vec<Wifi> {
        let name = ".nmconnection";
        let mut config: Vec<Wifi> = vec![];
        for net in networks.iter() {
            let string = net.to_string();
            let re: Regex = Regex::new(r"\[(?P<nombre>.*)\]\n(?P<valores>(.*=.*\n)*)").unwrap();
            if string.ends_with(name) {
                let text = String::from(DIR_NETWORK) + &string;
                let content = show_content(text.clone(), self.get_password().stdout);
                let mut w = Wifi::new();

                for cap in re.captures_iter(content.as_str()) {
                    let sec = Seccion::make(&cap[1], &cap[2]);
                    match sec.name.as_str() {
                        "wifi" => {
                            w.set_name(sec.get_attribute("ssid"));
                        },
                        "wifi-security" => {
                            w.set_password(sec.get_attribute("psk"));
                        },
                        _ => {}
                    }
                }
                config.push(w);
            }
        }

        config
    }
}

impl Lister for NetworkManager {
    fn list(&self) -> Vec<Wifi> {
        let args = vec!["-S", "-p", "''", "ls", DIR_NETWORK];
        let resultado = exec("sudo".to_owned(), args, self.get_password().stdout);
        let mut output = String::new();
        resultado.stdout.unwrap().read_to_string(&mut output).unwrap();

        let vec_networks: Vec<_> = output.split("\n").collect();
        let vec_networks: Vec<Wifi> = self.filter_networks(vec_networks);

        vec_networks
    }
}

