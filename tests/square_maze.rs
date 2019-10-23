extern crate maze;

use maze::square_maze::{SquareMaze, WallDirection};

#[test]
fn should_have_all_cells_walled_after_init() {

    let maze = SquareMaze::new_filled(10, 10);

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
fn minimal_maze_carving() {
    let mut maze = SquareMaze::new_filled(2, 2);
    for x in 0..1 {
        for y in 0..1 {
            assert!(maze.wall(WallDirection::NORTH, x, y));
            assert!(maze.wall(WallDirection::SOUTH, x, y));
            assert!(maze.wall(WallDirection::EAST, x, y));
            assert!(maze.wall(WallDirection::WEST, x, y));
        }
    }

    maze.carve(WallDirection::SOUTH, 0, 0);
    assert!(!maze.wall(WallDirection::SOUTH, 0, 0));
    assert!(!maze.wall(WallDirection::NORTH, 0, 1));

}

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_on_wall_access() {
    let maze = SquareMaze::new_filled(10, 10);
    maze.wall(WallDirection::NORTH, 11, 11);
}

#[test]
fn should_carve_some_walls() {
    let mut maze = SquareMaze::new_filled(10, 10);

    maze.carve(WallDirection::NORTH, 0, 0);
    assert!(!maze.wall(WallDirection::NORTH, 0, 0));
    assert!(maze.wall(WallDirection::SOUTH, 0, 0));
    assert!(maze.wall(WallDirection::EAST, 0, 0));
    assert!(maze.wall(WallDirection::WEST, 0, 0));

    maze.carve(WallDirection::EAST, 8, 8);
    assert!(maze.wall(WallDirection::NORTH, 8, 8));
    assert!(maze.wall(WallDirection::SOUTH, 8, 8));
    assert!(!maze.wall(WallDirection::EAST, 8, 8));
    assert!(maze.wall(WallDirection::WEST, 8, 8));
    assert!(!maze.wall(WallDirection::WEST, 9, 8));

    maze.carve(WallDirection::SOUTH, 5, 1);
    assert!(maze.wall(WallDirection::NORTH, 5, 1));
    assert!(!maze.wall(WallDirection::SOUTH, 5, 1));
    assert!(maze.wall(WallDirection::EAST, 5, 1));
    assert!(maze.wall(WallDirection::WEST, 5, 1));
    assert!(!maze.wall(WallDirection::NORTH, 5, 2));

    maze.carve(WallDirection::WEST, 0, 0);
    assert!(!maze.wall(WallDirection::NORTH, 0, 0));
    assert!(maze.wall(WallDirection::SOUTH, 0, 0));
    assert!(maze.wall(WallDirection::EAST, 0, 0));
    assert!(!maze.wall(WallDirection::WEST, 0, 0));
}

#[test]
fn should_carve_edge_walls() {
    let mut maze = SquareMaze::new_filled(10, 10);

    maze.carve(WallDirection::EAST, 9, 0);
    assert!(!maze.wall(WallDirection::EAST, 9, 0));

    maze.carve(WallDirection::SOUTH, 9, 9);
    assert!(!maze.wall(WallDirection::SOUTH, 9, 9));
}

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_on_wall_carving() {
    let mut maze = SquareMaze::new_filled(10, 10);
    maze.carve(WallDirection::NORTH, 11, 11);
}

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_on_neighbours() {
    let maze = SquareMaze::new_filled(10, 10);
    maze.neighbours(11, 11);
}

#[test]
fn should_get_neighbours() {
    let maze = SquareMaze::new_filled(10, 10);
    assert_eq!(maze.neighbours(0,0), [WallDirection::NORTH, WallDirection::EAST]);
    assert_eq!(maze.neighbours(0,1), [WallDirection::NORTH, WallDirection::EAST, WallDirection::SOUTH]);
    assert_eq!(maze.neighbours(0,9), [WallDirection::EAST, WallDirection::SOUTH]);
    assert_eq!(maze.neighbours(5,9), [WallDirection::EAST, WallDirection::SOUTH, WallDirection::WEST]);
    assert_eq!(maze.neighbours(9,9), [WallDirection::SOUTH, WallDirection::WEST]);
    assert_eq!(maze.neighbours(9,5), [WallDirection::NORTH, WallDirection::SOUTH, WallDirection::WEST]);
    assert_eq!(maze.neighbours(9,0), [WallDirection::NORTH, WallDirection::WEST]);
    assert_eq!(maze.neighbours(5,0), [WallDirection::NORTH, WallDirection::EAST, WallDirection::WEST]);

    assert_eq!(maze.neighbours(5,5), [WallDirection::NORTH, WallDirection::EAST, WallDirection::SOUTH, WallDirection::WEST]);
}
