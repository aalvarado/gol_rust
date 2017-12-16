#[derive(Clone)]
pub struct Cell {
    pub status: usize
}

impl Cell {
    pub fn new() -> Cell {
        Cell { status: 0 }
    }

    pub fn die(&mut self) -> usize {
        self.status = 0;
        self.status
    }

    pub fn spawn(&mut self) -> usize {
        self.status = 1;
        self.status
    }
}
