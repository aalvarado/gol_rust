extern crate toml;

use std::fs::File;
use std::io::prelude::*;

const PATH: &'static str = "conf/config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub width: i32,
    pub height: i32
}

impl Config {
    pub fn new() -> Config {
        toml::from_str(&Self::read_file()).unwrap()
    }

    pub fn read_file() -> String {
        let mut contents = String::new();
        let mut file = File::open(PATH).expect("Unable to open file");
        file.read_to_string(&mut contents).expect("Error reading config");
        contents
    }
}
