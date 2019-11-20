extern crate maze;

use maze::plm;
use maze::square_maze::{SquareMaze, MazeCell, WallDirection};
use maze::gen;

//write

#[test]
fn test_output_plm() {
    let mut maze = SquareMaze::new_filled_with_entry_exit(2, 2, MazeCell::new(0, 0), MazeCell::new(1, 1));
    maze.carve(WallDirection::EAST, 0, 0);

    let data = plm::output_to_buf(&maze);

    assert_eq!(11+1+1, data.len());
    assert_eq!('p' as u8, data[0]);
    assert_eq!('l' as u8, data[1]);
    assert_eq!('m' as u8, data[2]);
    assert_eq!(0x01, data[3]); //version
    assert_eq!(0x00, data[4]); //flags
    assert_eq!(2, data[5]);
    assert_eq!(2, data[6]);
    assert_eq!(0, data[7]);
    assert_eq!(0, data[8]);
    assert_eq!(1, data[9]);
    assert_eq!(1, data[10]);

    assert_eq!(0b11111111, data[11]);
    assert_eq!(0b11111101, data[12], "exp 0b10111111, actual {:b}", data[12]);
}

// read

#[test]
fn test_output_and_read_back_tiny_maze() {
    let mut maze = SquareMaze::new_filled_with_entry_exit(2, 2, MazeCell::new(0, 0), MazeCell::new(1, 1));
    maze.carve(WallDirection::EAST, 0, 0);

    let out_res = plm::output_to_buf(&maze);
    let read_back = plm::read_from_buf(&out_res);
    assert!(read_back.is_ok(), "read error = {:?}", read_back);

    assert_eq!(read_back.unwrap(), maze);
}

#[test]
fn test_output_and_read_back_tiny_maze_non_cubic() {
    let mut maze = SquareMaze::new_filled_with_entry_exit(2, 3, MazeCell::new(0, 0), MazeCell::new(1, 1));
    maze.carve(WallDirection::EAST, 0, 0);

    let out_res = plm::output_to_buf(&maze);
    let read_back = plm::read_from_buf(&out_res);
    assert!(read_back.is_ok(), "read error = {:?}", read_back);

    assert_eq!(read_back.unwrap(), maze);
}

#[test]
fn test_output_and_read_back_bigger_maze() {
    let mut maze = SquareMaze::new_filled_with_entry_exit(19, 25, MazeCell::new(0, 0), MazeCell::new(19, 25));
    let seed = [1;16];
    gen::recursive(&mut maze, seed, MazeCell::new(0, 0));

    let out_res = plm::output_to_buf(&maze);
    let read_back = plm::read_from_buf(&out_res);
    assert!(read_back.is_ok(), "read error = {:?}", read_back);

    assert_eq!(read_back.unwrap(), maze);
}
