use super::square_maze::{SquareMaze, WallDirection};
use std::fs::OpenOptions;
use std::io::Write;

pub fn tikz(out_file: &str, maze: &SquareMaze) {
    let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(out_file).unwrap();

    write!(file, "\\documentclass{{minimal}}\n").unwrap();
    write!(file, "\\usepackage{{tikz}}\n").unwrap();
    write!(file, "\\begin{{document}}\n").unwrap();
    write!(file, "\\pagestyle{{empty}}\n").unwrap();
    write!(file, "\n").unwrap();
    write!(file, "\\begin{{tikzpicture}}\n").unwrap();

    for x in 0..maze.width {
        for y in 0..maze.height {
            //south-wall
            if maze.wall(WallDirection::SOUTH, x,y) {
                file.write(line(x,y, x+1,y).as_bytes()).unwrap();
            }

            //west-wall
            if maze.wall(WallDirection::WEST, x, y) {
                file.write(line(x,y, x,y+1).as_bytes()).unwrap();
            }

            if x == maze.width-1 && maze.wall(WallDirection::EAST, x, y) {
                file.write(line(x+1,y, x+1,y+1).as_bytes()).unwrap();
            }

            if y == maze.height-1 && maze.wall(WallDirection::NORTH, x, y) {
                file.write(line(x,y+1, x+1,y+1).as_bytes()).unwrap();
            }
        }
    }

    write!(file, "\\end{{tikzpicture}}\n").unwrap();
    write!(file, "\\end{{document}}\n").unwrap();
}

fn line(x_from: usize, y_from: usize, x_to: usize, y_to: usize) -> String {
    return format!("\\draw ({},{}) -- ({},{});\n", x_from, y_from, x_to, y_to);
}
