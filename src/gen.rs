extern crate rand;

use super::square_maze::{SquareMaze};
use self::rand::{XorShiftRng, SeedableRng, Rng};

// TODO Impl generation algorithm (recursive!)
// https://doc.rust-lang.org/rand/rand/struct.XorShiftRng.html
pub fn recursive(maze: &mut SquareMaze, seed: [u32; 4]) {
    let mut rnd = XorShiftRng::from_seed(seed);

    let mut x = rnd.gen_range(0, maze.width);
    let mut y = rnd.gen_range(0, maze.height);
    let mut need_to_visit = maze.width * maze.height - 1;
    maze.mark_visited(x, y);

    while need_to_visit > 0 {
        let neighbours = maze.neighbours(x, y);
        let unvisited_neighbours = neighbours.iter().filter(|n| !maze.visited_neighbour(x, y, **n)).collect::<Vec<_>>();
        let walk = rnd.choose(&unvisited_neighbours);

        //TODO some more test for visited_neighbour function!!
        
        println!("#### walk {:?}", walk);
        //TODO calc neighbour x,y from neighbour and mark visited
        //maze.neighbour_pos(walk, x, y) => {x, y}

        x = rnd.gen_range(0, maze.width);
        y = rnd.gen_range(0, maze.height);
        need_to_visit = need_to_visit-1;
    }
}
