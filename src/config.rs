extern crate toml;
extern crate serde;

use std::path::Path;
use std::fs::File;

const PATH: &'static str = "../conf/config.tml";

#[derive(Deserialize)]
pub struct Config<'a> {
    pub width: u8,
    pub height: u8

}

impl <'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Config { width: 0, height: 0}
    }

    fn read_file() -> &'a str {
        let mut contents = String::new();
        File::open(PATH)?.read_to_string(contents);
        contents
    }
}
