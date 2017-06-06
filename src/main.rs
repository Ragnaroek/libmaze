extern crate maze;

use maze::square_maze::SquareMaze;
use maze::gen;
use maze::out;
use maze::meta::{to_hex_string, MetaData, all_stats};

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
    let mut max_meta = MetaData::new_empty();
    let mut max_maze = SquareMaze::new(0, 0);

    for i in 1..2 {
        let mut maze = SquareMaze::new(19, 25);
        let seed = [i, 1, 1, 1];
        let mut meta = MetaData::new_empty();
        meta.seed = to_hex_string(seed).to_string();
        gen::recursive(&mut maze, seed);
        all_stats(&mut maze, &mut meta);

        if meta.dead_ends > max_meta.dead_ends {
            max_meta = meta;
            max_maze = maze;
        }
    }

    out::tikz("/Users/mb/_libmazetest/test.tex", &max_maze, &max_meta);
}
