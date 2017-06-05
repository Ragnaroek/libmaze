
use super::square_maze::{WallDirection, dir_ix_x, dir_ix_y};

pub struct Visited {
    width: usize,
    height: usize,
    visited: Vec<u8>
}

impl Visited {
    pub fn new(width: usize, height: usize) -> Visited {
        let visited_size = ((width*height)/8)+1;
        let mut visited = Vec::with_capacity(visited_size);
        for i in 0..visited_size {
            visited.insert(i, 0);
        }

        return Visited{width, height, visited};
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
