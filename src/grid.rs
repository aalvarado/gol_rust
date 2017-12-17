use cell::Cell;

const COORDS: [(isize,isize); 8] = [
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0, -1),
    ( 0,  1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1)
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

    pub fn neighborg_count(&self, x: usize, y: usize) -> usize {
        let xy = (x as isize, y as isize);
        let height = self.height;
        let width = self.width;

        COORDS.iter().fold(0, |sum, &coord| {
            let new_coords = (xy.0 + coord.0, xy.1 + coord.1);
            let x_in_bounds = new_coords.0 >= 0 && new_coords.0 <= height;
            let y_in_bounds = new_coords.1 >= 0 && new_coords.1 <= width;

            if x_in_bounds && y_in_bounds {
                let new_x = new_coords.0 as usize;
                let new_y = new_coords.1 as usize;
                let cell = &self.cells[new_x][new_y];
                sum + cell.status
            } else {
                0
            }
        })
    }

    pub fn coord_check(&self, x: usize, y: usize) -> Result<(usize, usize), ()> {
        let height = self.height as usize;
        let width = self.width as usize;

        let x_in_bounds = x <= height;
        let y_in_bounds = y <= width;

        if x_in_bounds && y_in_bounds {
            Ok((x, y))
        } else {
            Err(())
        }
    }

    pub fn kill_cell(&mut self, x: usize, y: usize) -> usize {
        self.cells[x][y].die()
    }

    pub fn spawn_cell(&mut self, x: usize, y: usize) -> usize {
        self.cells[x][y].die()
    }

    pub fn scan_cells(&mut self) -> () {
        let mut row_index = 0usize;
        let mut cell_index = 0usize;
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                //match neighborg_count(row_index, cell_index)
                cell_index += 1;
            }
            cell_index = 0;
            row_index += 1;
        }
    }
}
