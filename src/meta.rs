// Calculates statistics about mazes
use super::square_maze::{MazeCell, SquareMaze, WallDirection, dir_ix_x, dir_ix_y};
use super::visited::Visited;

use std::collections::HashSet;

pub fn to_hex_string(seed: [u8; 16]) -> String {
    return format!(
        "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}",
        seed[0],
        seed[1],
        seed[2],
        seed[3],
        seed[4],
        seed[5],
        seed[6],
        seed[7],
        seed[8],
        seed[9],
        seed[10],
        seed[11],
        seed[12],
        seed[13],
        seed[14],
        seed[15]
    );
}

pub struct MetaData {
    pub seed: String,
    pub dead_ends: u32,
    pub distance: Distance,
}

impl MetaData {
    pub fn new_empty() -> MetaData {
        return MetaData {
            seed: "".to_string(),
            dead_ends: 0,
            distance: Distance::new(0, 0),
        };
    }
}

pub struct Distance {
    pub width: usize,
    pub height: usize,
    pub dist: Vec<Vec<u32>>,
}

impl Distance {
    pub fn new(width: usize, height: usize) -> Distance {
        let mut dist = Vec::with_capacity(width);
        for x in 0..width {
            let mut h_vec = Vec::with_capacity(height);
            for y in 0..height {
                h_vec.insert(y, 0);
            }
            dist.insert(x, h_vec);
        }

        return Distance {
            width: width,
            height: height,
            dist: dist,
        };
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
pub fn distances(maze: &SquareMaze, start: &MazeCell) -> Distance {
    let mut distances = Distance::new(maze.width, maze.height);
    let mut visited = Visited::new(maze.width, maze.height);
    let mut frontier: HashSet<MazeCell> = HashSet::new();
    frontier.insert(MazeCell::new(start.x, start.y));
    let mut dist = 0;

    while !frontier.is_empty() {
        let mut next_frontier: HashSet<MazeCell> = HashSet::new();

        for cell in frontier {
            distances.set(cell.x, cell.y, dist);
            visited.mark_visited(cell.x, cell.y);
            for neighbour in maze.neighbours(cell.x, cell.y) {
                let n_cell =
                    MazeCell::new(dir_ix_x(cell.x, *neighbour), dir_ix_y(cell.y, *neighbour));
                if !maze.wall(*neighbour, cell.x, cell.y) && !visited.visited(n_cell.x, n_cell.y) {
                    next_frontier.insert(n_cell);
                }
            }
        }

        dist = dist + 1;
        frontier = next_frontier;
    }

    return distances;
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
    meta.distance = distances(maze, &maze.entry);
}
