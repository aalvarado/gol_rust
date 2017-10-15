extern crate pancurses;

use self::pancurses::*;
use ::Config;

pub struct Screen<'a> {
    window: pancurses::Window,
    config: &'a::Config
}

impl<'a> Screen<'a> {
    pub fn new(config: &Config) -> Screen {
        initscr();
        let window = newwin(0, 0, 0, 0);
        let temp_self = Screen { window: window, config: config };
        temp_self
    }

    pub fn run(&self) {
        let window = &self.window;
        window.draw_box(0, 0);
        window.getch();
        window.refresh();
    }

    pub fn start(&self) -> () {
        ()
    }

    pub fn end(&self) -> () {
        ()
    }

    //pub fn init(&self) {
        ////let ref window = self.window;
        //const window = self.window;
        ////const config = self.config;
        //window.box(100, 100);
        //window.refresh();
    //}

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn height(&self) -> i32 {
        self.config().height
    }

    pub fn width(&self) -> i32 {
        self.config().width
    }

    pub fn talk(&self, str1: &str) {
        self.window.printw(str1);
        self.window.getch();
    }

    fn window(&self) -> &pancurses::Window {
        &self.window
    }
}

impl<'a> Drop for Screen<'a> {
    fn drop(&mut self) {
        endwin();
    }
}
