#![allow(dead_code)]

mod screen;
mod config;
mod grid;
mod cell;

#[macro_use]
extern crate serde_derive;

use screen::Screen;
use config::Config;
use grid::Grid;

fn main() {
    let config = Config::new();
    Screen::new(&config);
}
