extern crate pancurses;

use self::pancurses::{initscr, endwin};

pub struct Screen {
    window: pancurses::Window
}

impl Screen {
    pub fn new() -> Screen {
        Screen { window: initscr() }
    }

    pub fn talk(&self) {
        let ref window = self.window;

        window.printw("Hello!");
        window.refresh();
        window.getch();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        endwin();
    }
}
