#[derive(Clone)]
pub struct Cell {
    status: u8
}

impl Cell {
    pub fn new() -> Cell {
        Cell { status: 0 }
    }
}
