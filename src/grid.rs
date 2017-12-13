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
                //println!("{}", cell.status);
                //sum += cell.status
                sum + cell.status
            } else {
                0
            }
        })
    }
}
