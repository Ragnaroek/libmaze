extern crate rand;

use super::square_maze::{SquareMaze, WallDirection};
use self::rand::{XorShiftRng, SeedableRng, Rng};

// TODO Impl generation algorithm (recursive!)
// https://doc.rust-lang.org/rand/rand/struct.XorShiftRng.html
pub fn recursive(mut maze: &SquareMaze, seed: [u32; 4]) {
    let mut rnd = rand::XorShiftRng::from_seed(seed);
    //let choice = rnd.choose(&DIRS);
    //println!("rnd {:?}", choice);
}
