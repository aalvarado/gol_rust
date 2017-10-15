#![allow(dead_code)]

mod screen;
mod config;
mod logic;

#[macro_use]
extern crate serde_derive;

use screen::Screen;
use config::Config;
use logic::Logic;

fn main() {
    let config = Config::new();
    let screen = Screen::new(&config);
    let logic = Logic::new(&config);

    screen.start();
    screen.run();
    screen.end();
}
