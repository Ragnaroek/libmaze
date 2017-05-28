extern crate rand;

use super::square_maze::{SquareMaze, dir_ix_x, dir_ix_y};
use self::rand::{XorShiftRng, SeedableRng, Rng};

pub fn recursive(maze: &mut SquareMaze, seed: [u32; 4]) {
    let mut rnd = XorShiftRng::from_seed(seed);

    let mut x = rnd.gen_range(0, maze.width);
    let mut y = rnd.gen_range(0, maze.height);
    let mut need_to_visit = maze.width * maze.height - 1;
    maze.mark_visited(x, y);

    let mut x_stack = Vec::new();
    let mut y_stack = Vec::new();
    x_stack.push(x);
    y_stack.push(y);

    while need_to_visit > 0 {
        let neighbours = maze.neighbours(x, y);
        let unvisited_neighbours = neighbours.iter().filter(|n| !maze.visited_neighbour(x, y, **n)).collect::<Vec<_>>();
        let walk = rnd.choose(&unvisited_neighbours);

        maze.mark_visited(x, y);

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
