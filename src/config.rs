extern crate toml;
extern crate serde;

#[derive(Deserialize)]
pub struct Config {
    width: u8,
    height: u8
}

impl Config {
    pub fn new() -> Config {
        Config { width: 0, height: 0 }
    }
}
