
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum WallDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

pub struct SquareMaze {
    horizontal_walls: Vec<u8>,
    vertical_walls: Vec<u8>,
    pub width: usize,
    pub height: usize
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

impl SquareMaze {
    pub fn new(width: usize, height: usize) -> SquareMaze {

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

        return SquareMaze{horizontal_walls: h_walls, vertical_walls: v_walls, width, height};
    }

    pub fn wall(&self, dir: WallDirection, x: usize, y: usize) -> bool {
        self.check_bounds(x, y);

        match dir {
            WallDirection::WEST  => wall_bit_set(&self.horizontal_walls, x+y*self.width),
            WallDirection::EAST  => wall_bit_set(&self.horizontal_walls, (x+1)+y*self.width),
            WallDirection::SOUTH => wall_bit_set(&self.vertical_walls, y+x*self.height),
            WallDirection::NORTH => wall_bit_set(&self.vertical_walls, (y+1)+x*self.height)
        }
    }

    pub fn carve(&mut self, dir: WallDirection, x: usize, y: usize) {
        self.check_bounds(x, y);

        match dir {
            WallDirection::WEST  => unset_wall_bit(&mut self.horizontal_walls, x+y*self.width),
            WallDirection::EAST  => unset_wall_bit(&mut self.horizontal_walls, (x+1)+y*self.width),
            WallDirection::SOUTH => unset_wall_bit(&mut self.vertical_walls, y+x*self.height),
            WallDirection::NORTH => unset_wall_bit(&mut self.vertical_walls, (y+1)+x*self.height)
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
        if x >= self.width {
            panic!("Out of bound cell access {} >= width {}", x, self.width)
        }
        if y >= self.height {
            panic!("Out of bound cell access {} >= height {}", y, self.height)
        }
    }
}
