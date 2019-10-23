extern crate rand;
extern crate rand_xorshift;

use super::square_maze::{SquareMaze, WallDirection, MazeCell, dir_ix_x, dir_ix_y};
use super::visited::{Visited};
use self::rand::{SeedableRng, Rng};
use self::rand::seq::{SliceRandom};
use self::rand_xorshift::{XorShiftRng};

pub fn recursive(maze: &mut SquareMaze, seed: [u8; 16], start: MazeCell) {
    let mut rnd = XorShiftRng::from_seed(seed);
    let mut visit = Visited::new(maze.width, maze.height);

    //start position is a random position at the top of the maze
    let mut x = start.x;
    let mut y = start.y;
    maze.carve(WallDirection::NORTH, x, y);

    let mut need_to_visit = maze.width * maze.height - 1;
    visit.mark_visited(x, y);

    let mut x_stack = Vec::new();
    let mut y_stack = Vec::new();
    x_stack.push(x);
    y_stack.push(y);

    // TODO loop until stack is empty! (need_to_visit is redundant)
    while need_to_visit > 0 {
        let neighbours = maze.neighbours(x, y);
        let unvisited_neighbours = neighbours.iter().filter(|n| !visit.visited_neighbour(x, y, **n)).collect::<Vec<_>>();
        let walk = unvisited_neighbours.choose(&mut rnd);

        visit.mark_visited(x, y);

        match walk {
            Some(d) => {
                maze.carve(**d, x, y);
                x = dir_ix_x(x, **d);
                y = dir_ix_y(y, **d);

                x_stack.push(x);
                y_stack.push(y);

                need_to_visit = need_to_visit-1;
            },
            None => {
                let x_ = x_stack.pop();
                let y_ = y_stack.pop();
                if x_.is_none() || y_.is_none() {
                    //done!
                    break;
                } else {
                    x = x_.unwrap();
                    y = y_.unwrap();
                }
            }
        }
    }
}
