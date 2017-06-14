extern crate toml;

use std::fs::File;
use std::io::prelude::*;

const PATH: &'static str = "conf/config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub width: u8,
    pub height: u8
}

impl Config {
    pub fn new() -> Config {
        toml::from_str(&Self::read_file()).unwrap()
    }

    pub fn read_file() -> String {
        let mut contents = String::new();
        let mut file = match File::open(PATH) {
            Ok(file) => file,
            Err(_) => {
                panic!("Could not find config file");
            }
        };

        file.read_to_string(&mut contents).unwrap_or_else(|_| panic!("Error reading config") );
        contents
    }
}
