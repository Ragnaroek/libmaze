// Calculates statistics about mazes
use super::square_maze::{SquareMaze, WallDirection};

pub fn to_hex_string(seed: [u32; 4]) -> String {
    return format!("{:08X}-{:08X}-{:08X}-{:08X}", seed[0], seed[1], seed[2], seed[3]);
}

pub struct MetaData {
    pub seed: String,
    pub dead_ends: u32
}

impl MetaData {
    pub fn new_empty() -> MetaData {
        return MetaData{seed: "".to_string(), dead_ends: 0};
    }
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
}
