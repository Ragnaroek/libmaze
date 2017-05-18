extern crate maze;

use maze::square_maze::SquareMaze;
use maze::gen;

fn main() {
    let mut maze = SquareMaze::new(10, 10);
    let seed = [1, 1, 1, 1];
    gen::recursive(&mut maze, seed);
}
