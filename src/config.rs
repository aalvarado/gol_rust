extern crate toml;
extern crate serde;

use std::path::Path;


#[derive(Deserialize)]
pub struct Config {
    pub width: u8,
    pub height: u8

}

impl Config {
    pub fn new() -> Config {
        Config { width: 0, height: 0}
    }
}
