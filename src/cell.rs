#[derive(Clone)]
pub struct Cell {
    pub status: usize
}

impl Cell {
    pub fn new() -> Cell {
        Cell { status: 0 }
    }
}
