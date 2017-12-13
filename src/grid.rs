use cell::Cell;

const COORDINATES: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
];

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    height: 0,
    width: 0
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        Grid {
            cells: vec![vec![Cell::new(); width]; height],
            height: height,
            width: width
        }
    }

    pub fn coords(&self, x: usize, y: usize) -> () {
    }

    pub fn neighborg_count(&self) -> () {
    }
}
