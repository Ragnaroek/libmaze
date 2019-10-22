
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum WallDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

fn wall_bit_set(walls: &Vec<u8>, i: usize) -> bool {
    let byte_ix = i/8;
    let bit_ix = i%8;
    let byte = walls[byte_ix];
    return (byte & (1 << bit_ix)) != 0;
}

fn unset_wall_bit(walls: &mut Vec<u8>, i: usize) {
    let byte_ix = i/8;
    let bit_ix = i%8;
    let byte = walls[byte_ix];
    walls[byte_ix] = byte & !(1 << bit_ix)
}

pub fn dir_ix_x(x: usize, n: WallDirection) -> usize {
    match n {
        WallDirection::WEST => x - 1,
        WallDirection::EAST => x + 1,
        _ => x
    }
}

pub fn dir_ix_y(y: usize, n: WallDirection) -> usize {
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

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct MazeCell {
   pub x: usize,
   pub y: usize
}

impl MazeCell {
    pub fn new(x: usize, y: usize) -> MazeCell {
        return MazeCell{x,y};
    }
}

//2x2 maze

//    0h   1h
//0v|---|1v---|3v    (0,0) NORTH y*w + x = 0
//  |2h |3h   |      (0,0) SOUTH == (0,1) NORTH = 1*2 + 0 = 2
//  |---|-----|      (0,0) WEST =
//  |4h |5h   |      (0,0) EAST = (0,1) WEST
//  |---|-----|

#[derive(PartialEq, Debug)]
pub struct SquareMaze {
   horizontal_walls: Vec<u8>,
   vertical_walls: Vec<u8>,
   pub width: usize,
   pub height: usize,
   pub entry: MazeCell,
   pub exit: MazeCell
}

impl SquareMaze {
    pub fn new(width: usize, height: usize, entry: MazeCell, exit: MazeCell, horizontal_walls: Vec<u8>, vertical_walls: Vec<u8>) -> SquareMaze {
        SquareMaze{horizontal_walls, vertical_walls, width, height, entry, exit}
    }

    pub fn new_filled_with_entry_exit(width: usize, height: usize, entry: MazeCell, exit: MazeCell) -> SquareMaze {
        let h_size = (((width+1)*height)/8)+1;
        let v_size = (((height+1)*width)/8)+1;

        let mut h_walls = Vec::with_capacity(h_size);
        for i in 0..h_size {
            h_walls.insert(i, 255);
        }

        let mut v_walls = Vec::with_capacity(v_size);
        for i in 0..v_size {
            v_walls.insert(i, 255);
        }

        SquareMaze{horizontal_walls: h_walls,
                   vertical_walls: v_walls,
                   width,
                   height,
                   entry,
                   exit}
    }

    pub fn new_filled(width: usize, height: usize) -> SquareMaze {
        let entry = MazeCell::new(0, 0);
        let exit = MazeCell::new(0, 0);
        SquareMaze::new_filled_with_entry_exit(width, height, entry, exit)
    }

    pub fn wall(&self, dir: WallDirection, x: usize, y: usize) -> bool {
        self.check_bounds(x, y);


        //0v|---|1v---|3v    (0,0) NORTH y*w + x = 0
        //  |2h |3h   |      (0,0) SOUTH == (0,1) NORTH = 1*2 + 0 = 2
        //  |---|-----|      (0,0) WEST = y*w + x
        //  |4h |5h   |      (0,0) EAST = (1,0) WEST
        //0,0 SOUTH == 0,1 NORTH ?
        //0+0*h = 0 == 2+0*h = 2
        match dir {
            WallDirection::WEST  => wall_bit_set(&self.vertical_walls, x+y*self.width),
            WallDirection::EAST  => self.wall(WallDirection::WEST, x+1, y),
            WallDirection::SOUTH => self.wall(WallDirection::NORTH, x, y+1),
            WallDirection::NORTH => wall_bit_set(&self.horizontal_walls, x+y*self.width)
        }
    }

    pub fn carve(&mut self, dir: WallDirection, x: usize, y: usize) {
        self.check_bounds(x, y);

        match dir {
            WallDirection::WEST  => unset_wall_bit(&mut self.vertical_walls, x+y*self.width),
            WallDirection::EAST  => self.carve(WallDirection::WEST, x+1, y),
            WallDirection::SOUTH => self.carve(WallDirection::NORTH, x, y+1),
            WallDirection::NORTH => unset_wall_bit(&mut self.horizontal_walls, x+y*self.width)
        }
    }

    // y=height-1 00000000000000
    //
    //
    // x=0        00000000000000 x=width-1
    pub fn neighbours(&self, x: usize, y: usize) -> & 'static [WallDirection] {
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

    fn check_bounds(&self, x: usize, y: usize) -> () {
        if x >= self.width+1 {
            panic!("Out of bound cell access {} >= width {}", x, self.width)
        }
        if y >= self.height+1 {
            panic!("Out of bound cell access {} >= height {}", y, self.height)
        }
    }
}
