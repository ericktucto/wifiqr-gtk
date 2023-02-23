use std::{ process::Child, io::Read };

use regex::Regex;

use crate::{commands::exec, wifi::show_content};

use super::{Lister, Wifi, Seccion};

const DIR_NETWORK: &str = "/var/lib/connman/";

pub struct ConnMan {
    pub password: String
}

impl ConnMan {
    fn get_password(&self) -> Child {
        let args: Vec<&str> = vec![self.password.as_str()];
        exec("echo".to_owned(), args, None)
    }
}

impl Lister for ConnMan {
    fn list(&self) -> Vec<Wifi> {
        let mut config: Vec<Wifi> = vec![];

        let args = vec![DIR_NETWORK, "-maxdepth", "1", "-type", "d", "-name", "wifi_*"];
        let resultado = exec("find".to_owned(), args, None);
        let mut output = String::new();
        resultado.stdout.unwrap().read_to_string(&mut output).unwrap();
        let list_wifi: Vec<_> = output.split("\n").collect();

        for path_wifi in list_wifi.iter() {
            if path_wifi.to_string() == "" {
                continue;
            }
            let file_setting = path_wifi.to_string() + "/settings";
            let content = show_content(file_setting, self.get_password().stdout);
            let mut w = Wifi::new();
            let re: Regex = Regex::new(r"\[(?P<nombre>.*)\]\n(?P<valores>(.*=.*\n)*)").unwrap();
            for cap in re.captures_iter(content.as_str()) {
                let sec = Seccion::make(&cap["nombre"], &cap["valores"]);
                w.set_name(sec.get_attribute("Name"));
                w.set_password(sec.get_attribute("Passphrase"));
            }
            config.push(w);
        }

        config
    }
}
