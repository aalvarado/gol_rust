mod screen;
use screen::Screen;

fn main() {
    let screen = Screen::new();
    screen.talk();
    //println!("address of screen: {:p}", &screen);
}
