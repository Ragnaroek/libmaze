extern crate maze;

use maze::core::{SquareMaze, WallDirection};

#[test]
fn should_have_all_cells_walled_after_init() {

    let maze = SquareMaze::new(10, 10);

    for x in 0..10 {
        for y in 0..10 {
            assert!(maze.wall(WallDirection::NORTH, x, y));
            assert!(maze.wall(WallDirection::SOUTH, x, y));
            assert!(maze.wall(WallDirection::EAST, x, y));
            assert!(maze.wall(WallDirection::WEST, x, y));
        }
    }
}

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range() {
    let maze = SquareMaze::new(10, 10);
    maze.wall(WallDirection::NORTH, 10, 10);
}
