// Calculates statistics about mazes
use super::square_maze::{SquareMaze, WallDirection};

pub fn to_hex_string(seed: [u32; 4]) -> String {
    return format!("{:08X}-{:08X}-{:08X}-{:08X}", seed[0], seed[1], seed[2], seed[3]);
}

pub struct MetaData {
    pub seed: String,
    pub dead_ends: u32,
    pub distance: Distance
}

impl MetaData {
    pub fn new_empty() -> MetaData {
        return MetaData{seed: "".to_string(), dead_ends: 0, distance: Distance::new_empty(0,0)};
    }
}

pub struct Distance {
    pub width: usize,
    pub height: usize,
    pub dist: Vec<Vec<u32>>
}

impl Distance {
    pub fn new_empty(width: usize, height: usize) -> Distance {
        let mut dist = Vec::with_capacity(width);
        for x in 0..width {
            let mut h_vec = Vec::with_capacity(height);
            for y in 0..height {
                h_vec.insert(y, 0);
            }
            dist.insert(x, h_vec);
        }

        return Distance{width: width, height: height, dist: dist}
    }

    pub fn distance(&self, x: usize, y: usize) -> u32 {
        return self.dist[x][y];
    }

    pub fn set(&mut self, x: usize, y: usize, dist: u32) {
        self.dist[x][y] = dist;
    }
}

///
/// Calculates the distance-matrix from a given start position
/// in the maze with the Dijkstra-Algorithm.
pub fn distances(maze: &SquareMaze, x: usize, y: usize) -> Distance {
    //TODO Actually implement algorithm :)
    return Distance::new_empty(maze.width, maze.height);
}

pub fn dead_ends(maze: &SquareMaze, meta: &mut MetaData) {
    for x in 0..maze.width {
        for y in 0..maze.height {
            if is_dead_end(x, y, maze) {
                meta.dead_ends = meta.dead_ends + 1;
            }
        }
    }
}

pub fn is_dead_end(x: usize, y: usize, maze: &SquareMaze) -> bool {
    let mut num_walls = 0;
    if maze.wall(WallDirection::NORTH, x, y) {
        num_walls = num_walls + 1;
    }
    if maze.wall(WallDirection::EAST, x, y) {
        num_walls = num_walls + 1;
    }
    if maze.wall(WallDirection::SOUTH, x, y) {
        num_walls = num_walls + 1;
    }
    if maze.wall(WallDirection::WEST, x, y) {
        num_walls = num_walls + 1;
    }
    return num_walls == 3;
}

pub fn all_stats(maze: &SquareMaze, meta: &mut MetaData) {
    dead_ends(maze, meta);
    // TODO use correct start position!
    meta.distance = distances(maze, 0, 0);
}
