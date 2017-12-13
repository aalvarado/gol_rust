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
    height: isize,
    width: isize
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        Grid {
            cells: vec![vec![Cell::new(); width]; height],
            height: height as isize,
            width: width as isize
        }
    }

    pub fn coords(&self, x: usize, y: usize) -> () {
    }

    pub fn neighborg_count(&self, x: isize, y: isize) -> usize {
        let width = self.width;
        let height = self.height;

        COORDINATES.iter().fold(0, |sum, coord| {
            let coords = (x + coord.0, y + coord.1);
            let x_in_bounds = x >= 0 && x <= width;
            let y_in_bounds = y >= 0 && y <= height;

            if x_in_bounds && y_in_bounds {
                sum += self.cells[coords.0][coords.1].status
            }
        })
    }
}
