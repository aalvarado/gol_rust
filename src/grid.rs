use gol_cell::GolCell;

pub struct Grid {
    cells: Vec<Vec<GolCell>>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        Grid {
            cells: vec![vec![GolCell::new(); width]; height],
        }
    }

    pub fn coords(x: usize, y: usize) -> Vec<(usize, usize)> {
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    }
}
