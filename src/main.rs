#![allow(dead_code)]

//mod screen;
mod config;

#[macro_use]
extern crate serde_derive;

//use screen::Screen;
use config::Config;

fn main() {
    let c = Config::new();
    //let x = c.read_file();
    println!("{}", c.width);
    println!("{}", c.height);
    //Screen::new(c);
}
