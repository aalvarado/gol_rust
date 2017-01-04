extern crate pancurses;

pub struct Screen {
    window: pancurses::Window
}

impl Screen {
    pub fn new() -> Screen {
        Screen { window: pancurses::initscr() }
    }

    pub fn talk(&self) {
        self.window.printw("Hello!");
        self.window.refresh();
        self.window.getch();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}
