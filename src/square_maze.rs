
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum WallDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

pub struct SquareMaze {
    cells: Vec<u8>,
    visited: Vec<u8>,
    pub width: usize,
    pub height: usize
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

fn dir_ix_x(x: usize, n: WallDirection) -> usize {
    match n {
        WallDirection::WEST => x - 1,
        WallDirection::EAST => x + 1,
        _ => x
    }
}

fn dir_ix_y(y: usize, n: WallDirection) -> usize {
    match n {
        WallDirection::NORTH => y + 1,
        WallDirection::SOUTH => y - 1,
        _ => y
    }
}

static SW_NBS  : [WallDirection; 2] = [WallDirection::NORTH, WallDirection::EAST];
static SE_NBS  : [WallDirection; 2] = [WallDirection::NORTH, WallDirection::WEST];
static NE_NBS  : [WallDirection; 2] = [WallDirection::SOUTH, WallDirection::WEST];
static NW_NBS  : [WallDirection; 2] = [WallDirection::EAST, WallDirection::SOUTH];
static N_NBS  : [WallDirection; 3] = [WallDirection::EAST, WallDirection::SOUTH, WallDirection::WEST];
static E_NBS  : [WallDirection; 3] = [WallDirection::NORTH, WallDirection::SOUTH, WallDirection::WEST];
static S_NBS  : [WallDirection; 3] = [WallDirection::NORTH, WallDirection::EAST, WallDirection::WEST];
static W_NBS  : [WallDirection; 3] = [WallDirection::NORTH, WallDirection::EAST, WallDirection::SOUTH];

static ALL_NBS : [WallDirection; 4] = [WallDirection::NORTH, WallDirection::EAST,
                                       WallDirection::SOUTH, WallDirection::WEST];

impl SquareMaze {
    pub fn new(width: usize, height: usize) -> SquareMaze {
        let size = ((width/2)+1)*height;
        let mut cells = Vec::with_capacity(size);
        for i in 0..size {
            cells.insert(i, 255);
        }

        let visited_size = ((width*height)/8)+1;
        let mut visited = Vec::with_capacity(visited_size);
        for i in 0..visited_size {
            visited.insert(i, 0);
        }
        return SquareMaze{cells, visited, width, height};
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

    // y=height-1 00000000000000
    //
    //
    // x=0        00000000000000 x=width-1
    pub fn neighbours(&self, x: usize, y: usize) -> &[WallDirection] {
        self.check_bounds(x, y);

        if x == 0 {
            if y == 0 {
                return &SW_NBS;
            }
            if y == self.height-1 {
                return &NW_NBS;
            }
            return &W_NBS;
        } else if x == self.width-1 {
            if y == 0 {
                return &SE_NBS;
            }
            if y == self.height-1 {
                return &NE_NBS;
            }
            return &E_NBS;
        }
        if y == self.height-1 {
            return &N_NBS;
        }
        if y == 0 {
            return &S_NBS
        }
        return &ALL_NBS;
    }

    pub fn visited(&self, x: usize, y: usize) -> bool {
        self.check_bounds(x, y);
        
        let bit_index = y * self.width + x;
        let byte_index = bit_index/8;
        let byte = self.visited[byte_index];
        let mask = 1 << (bit_index % 8);

        return (byte & mask) != 0;
    }
    pub fn visited_neighbour(&self, x: usize, y: usize, n: WallDirection) -> bool {
        return self.visited(dir_ix_x(x, n), dir_ix_y(y, n));
    }

    pub fn mark_visited(&mut self, x: usize, y: usize) {
        let bit_index = y * self.width + x;
        let byte_index = bit_index/8;
        let byte = self.visited[byte_index];
        let mask = 1 << (bit_index % 8);

        self.visited[byte_index] = byte | mask;
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
