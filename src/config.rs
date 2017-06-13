extern crate toml;
extern crate serde;

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
        Config { width: 0, height: 0}
    }

    pub fn read_file(&self) -> String {
        let mut contents = String::new();
        File::open(PATH).unwrap().read_to_string(&mut contents);
        contents
    }
}
