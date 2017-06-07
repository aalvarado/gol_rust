#![allow(dead_code)]

mod screen;
mod config;

#[macro_use]
extern crate serde_derive;

use screen::Screen;
use config::Config;

fn main() {
    let c = Config::new();
    Screen::new(c);
}
