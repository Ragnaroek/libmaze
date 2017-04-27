extern crate maze;

use maze::core::SquareMaze;

fn main() {
    let maze = SquareMaze::new(10, 10);
    println!("cell0 {}", maze.raw_cell0())
}
