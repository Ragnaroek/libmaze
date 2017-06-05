extern crate maze;

use maze::square_maze::{SquareMaze, WallDirection};
use maze::stat::{is_dead_end};

#[test]
fn should_be_dead_end_only_if_cell_has_three_walls() {
    let mut maze = SquareMaze::new(10, 10);

    assert!(!is_dead_end(0, 0, &maze));
    maze.carve(WallDirection::NORTH, 0, 0);
    assert!(is_dead_end(0, 0, &maze));
    maze.carve(WallDirection::EAST, 0, 0);
    assert!(!is_dead_end(0, 0, &maze));

    assert!(!is_dead_end(5, 5, &maze));
    maze.carve(WallDirection::SOUTH, 5, 5);
    assert!(is_dead_end(5, 5, &maze));
    maze.carve(WallDirection::WEST, 5, 5);
    assert!(!is_dead_end(5, 5, &maze));
    maze.carve(WallDirection::NORTH, 5, 5);
    assert!(!is_dead_end(5, 5, &maze));
    maze.carve(WallDirection::EAST, 5, 5);
    assert!(!is_dead_end(5, 5, &maze));
}
