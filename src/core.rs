
// TODO Impl wall carving
// TODO Impl generation algorithm (recursive!)

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum WallDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

pub struct SquareMaze {
    cells: Vec<u8>,
    width: usize,
    height: usize
}

impl SquareMaze {
    pub fn new(width: usize, height: usize) -> SquareMaze {
        let size = width*height;
        let mut cells = Vec::with_capacity(size);
        for i in 0..size {
            cells.insert(i, 255);
        }
        return SquareMaze{cells, width, height};
    }

    pub fn wall(&self, dir: WallDirection, x: usize, y: usize) -> bool {
        self.check_bounds(x, y);

        let cell = self.cells[y * (self.width/2) + x];
        if x.trailing_zeros() == 0 { //even
            println!("even")
        } else { //odd
            println!("odd")
        }

        print!("cell {}", cell);
        return false
    }

    fn check_bounds(&self, x: usize, y: usize) -> () {
        if x >= self.width {
            panic!("Out of bound cell access {} >= width {}", x, self.width)
        }
        if y >= self.height {
            panic!("Out of bound cell access {} >= height {}", y, self.height)
        }
    }

    //Only for testing!
    pub fn raw_cell0(self) -> u8 {
        return self.cells[0];
    }
}
