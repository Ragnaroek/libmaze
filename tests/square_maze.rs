extern crate maze;

use maze::square_maze::{SquareMaze, WallDirection};

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
fn should_panic_if_index_out_of_range_on_wall_access() {
    let maze = SquareMaze::new(10, 10);
    maze.wall(WallDirection::NORTH, 10, 10);
}

#[test]
fn should_carve_some_walls() {
    let mut maze = SquareMaze::new(10, 10);

    maze.carve(WallDirection::NORTH, 0, 0);
    assert!(!maze.wall(WallDirection::NORTH, 0, 0));
    assert!(maze.wall(WallDirection::SOUTH, 0, 0));
    assert!(maze.wall(WallDirection::EAST, 0, 0));
    assert!(maze.wall(WallDirection::WEST, 0, 0));

    maze.carve(WallDirection::EAST, 9, 9);
    assert!(maze.wall(WallDirection::NORTH, 9, 9));
    assert!(maze.wall(WallDirection::SOUTH, 9, 9));
    assert!(!maze.wall(WallDirection::EAST, 9, 9));
    assert!(maze.wall(WallDirection::WEST, 9, 9));

    maze.carve(WallDirection::SOUTH, 5, 1);
    assert!(maze.wall(WallDirection::NORTH, 5, 1));
    assert!(!maze.wall(WallDirection::SOUTH, 5, 1));
    assert!(maze.wall(WallDirection::EAST, 5, 1));
    assert!(maze.wall(WallDirection::WEST, 5, 1));

    maze.carve(WallDirection::WEST, 0, 0);
    assert!(!maze.wall(WallDirection::NORTH, 0, 0));
    assert!(maze.wall(WallDirection::SOUTH, 0, 0));
    assert!(maze.wall(WallDirection::EAST, 0, 0));
    assert!(!maze.wall(WallDirection::WEST, 0, 0));
}

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_on_wall_carving() {
    let mut maze = SquareMaze::new(10, 10);
    maze.carve(WallDirection::NORTH, 10, 10);
}

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_on_neighbours() {
    let mut maze = SquareMaze::new(10, 10);
    maze.neighbours(10, 10);
}

#[test]
fn should_get_neighbours() {
    let maze = SquareMaze::new(10, 10);
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

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_for_visited() {
    let mut maze = SquareMaze::new(5, 7);
    maze.visited(5, 7);
}

#[test]
fn should_have_initial_visited_state_of_false() {
    let maze = SquareMaze::new(10, 10);
    for x in 0..10 {
        for y in 0..10 {
            assert!(!maze.visited(x, y));
        }
    }
}

#[test]
fn should_maintain_visited_state() {
    let mut maze = SquareMaze::new(10, 10);
    assert!(!maze.visited(0,0));
    maze.mark_visited(0,0);
    assert!(maze.visited(0,0));

    assert!(!maze.visited(0,9));
    maze.mark_visited(0,9);
    assert!(maze.visited(0,9));

    assert!(!maze.visited(9,0));
    maze.mark_visited(9,0);
    assert!(maze.visited(9,0));

    assert!(!maze.visited(9,9));
    maze.mark_visited(9,9);
    assert!(maze.visited(9,9));
}

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_for_neighbours_unvisited() {
    let mut maze = SquareMaze::new(5, 5);
    maze.visited_neighbour(5, 5, WallDirection::NORTH);
}

#[test]
#[should_panic]
fn should_panic_if_neighbour_out_of_range_for_neighbours_unvisited() {
    let mut maze = SquareMaze::new(5, 5);
    maze.visited_neighbour(4, 4, WallDirection::NORTH);
}

#[test]
fn should_get_visited_state_of_neighbour() {
    let mut maze = SquareMaze::new(10, 10);
    maze.mark_visited(0, 1);
    assert!(maze.visited_neighbour(0, 0, WallDirection::NORTH));
    assert!(!maze.visited_neighbour(0, 0, WallDirection::EAST));

    maze.mark_visited(1, 0);
    assert!(maze.visited_neighbour(0, 0, WallDirection::EAST));
}
