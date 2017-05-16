
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

fn wall_bitmask(dir: WallDirection, upper: bool) -> u8 {
    let mut mask = match dir {
        WallDirection::NORTH => 0x01,
        WallDirection::EAST  => 0x02,
        WallDirection::SOUTH => 0x04,
        WallDirection::WEST  => 0x08,
    };
    if upper {
        mask = mask << 4;
    }
    return mask;
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

    fn cell_index(&self, x: usize, y: usize) -> usize {
        return y * (self.width/2) + x;
    }

    pub fn wall(&self, dir: WallDirection, x: usize, y: usize) -> bool {
        self.check_bounds(x, y);

        let cell = self.cells[self.cell_index(x, y)];
        let mask = wall_bitmask(dir, x%2!=0);
        return (cell & mask) != 0;
    }

    pub fn carve(&mut self, dir: WallDirection, x: usize, y: usize) {
        self.check_bounds(x, y);

        let ix = self.cell_index(x, y);
        let cell = self.cells[ix];
        let mask = !wall_bitmask(dir, x%2!=0);
        self.cells[ix] = cell & mask;
    }

    fn check_bounds(&self, x: usize, y: usize) -> () {
        if x >= self.width {
            panic!("Out of bound cell access {} >= width {}", x, self.width)
        }
        if y >= self.height {
            panic!("Out of bound cell access {} >= height {}", y, self.height)
        }
    }
}
