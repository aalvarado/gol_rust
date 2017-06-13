extern crate pancurses;

use self::pancurses::{initscr, endwin};
use ::Config;

pub struct Screen {
    window: pancurses::Window,
    //config: ::Config
}

impl Screen {
    pub fn new(config: Config) -> Screen {
        //let temp_self = Screen { window: initscr(), config: config };
        let temp_self = Screen { window: initscr() };
        temp_self.init();
        temp_self.talk("test");
        temp_self
    }

    pub fn init(&self) {
        //let ref window = self.window;
        self.window.refresh();
    }

    pub fn talk(&self, str1: &str) {
        self.window.printw(str1);
        self.window.getch();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        endwin();
    }
}
