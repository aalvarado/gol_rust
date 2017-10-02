extern crate pancurses;

use self::pancurses::{initscr, endwin};
use ::Config;
use ::Grid;

pub struct Screen<'a> {
    window: pancurses::Window,
    config: &'a Config,
    grid: ::Grid
}

impl<'a> Screen<'a> {
    pub fn new(config: &'a Config) -> Screen<'a> {
        let temp_self = Screen {
            window: initscr(),
            config: config,
            grid: Grid::new(config.height, config.width)
        };

        temp_self.init();
        temp_self
    }

    fn init(&self) {
        self.window.refresh();
    }

    pub fn talk(&self, str1: &str) {
        self.window.printw(str1);
        self.window.getch();
    }
}

impl <'a> Drop for Screen<'a> {
    fn drop(&mut self) {
        endwin();
    }
}
