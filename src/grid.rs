use cell::Cell;

pub struct Grid {
    cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        Grid { cells: vec![vec![Cell::new(); width]; height] }
    }
}
