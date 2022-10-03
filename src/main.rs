#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate config;

// mod config;
mod gol_cell;
mod grid;
mod screen;

// #[macro_use]
// extern crate serde_derive;

use config::Config;
// use screen::Screen;
// use grid::Grid;
// use screen::Screen;

// use config::Config;
// use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

fn main() {
    // Screen::new();
}
