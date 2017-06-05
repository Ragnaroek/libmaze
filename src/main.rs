extern crate maze;

use maze::square_maze::SquareMaze;
use maze::gen;
use maze::out;
use maze::stat;
use maze::maze::{to_hex_string, MetaData};

//A4 highest level: 63x93

//lvl0:
//19x25
//scale: 0.85

//lvl1:
//25x33
//scale: 0.65

//lvl2:
//36x47
//scale: 0.45

fn main() {
    let mut maze = SquareMaze::new(19, 25);
    let seed = [1, 1, 1, 1];
    let mut meta = MetaData::new_empty();
    meta.seed = to_hex_string(seed).to_string();
    gen::recursive(&mut maze, seed);
    stat::all(&mut maze, &mut meta);
    out::tikz("/Users/mb/_libmazetest/test.tex", &maze, &meta);
}
