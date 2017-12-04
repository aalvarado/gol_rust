use cell::Cell;

const COORDINATES: [(isize, isize)] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
]

pub struct Grid {
    cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        Grid { cells: vec![vec![Cell::new(); width]; height] }
    }

    pub fn coords(x: usize, y: usize) -> Vec<(usize, usize)> {

}
