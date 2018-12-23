extern crate maze;

use maze::visited::{Visited};
use maze::square_maze::{WallDirection};

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_for_visited() {
    let vis = Visited::new(5, 7);
    vis.visited(5, 7);
}

#[test]
fn should_have_initial_visited_state_of_false() {
    let vis = Visited::new(10, 10);
    for x in 0..10 {
        for y in 0..10 {
            assert!(!vis.visited(x, y));
        }
    }
}

#[test]
fn should_maintain_visited_state() {
    let mut vis = Visited::new(10, 10);
    assert!(!vis.visited(0,0));
    vis.mark_visited(0,0);
    assert!(vis.visited(0,0));

    assert!(!vis.visited(0,9));
    vis.mark_visited(0,9);
    assert!(vis.visited(0,9));

    assert!(!vis.visited(9,0));
    vis.mark_visited(9,0);
    assert!(vis.visited(9,0));

    assert!(!vis.visited(9,9));
    vis.mark_visited(9,9);
    assert!(vis.visited(9,9));
}

#[test]
#[should_panic]
fn should_panic_if_index_out_of_range_for_neighbours_unvisited() {
    let vis = Visited::new(5, 5);
    vis.visited_neighbour(5, 5, WallDirection::NORTH);
}

#[test]
#[should_panic]
fn should_panic_if_neighbour_out_of_range_for_neighbours_unvisited() {
    let vis = Visited::new(5, 5);
    vis.visited_neighbour(4, 4, WallDirection::NORTH);
}

#[test]
fn should_get_visited_state_of_neighbour() {
    let mut vis = Visited::new(10, 10);
    vis.mark_visited(0, 1);
    assert!(vis.visited_neighbour(0, 0, WallDirection::NORTH));
    assert!(!vis.visited_neighbour(0, 0, WallDirection::EAST));

    vis.mark_visited(1, 0);
    assert!(vis.visited_neighbour(0, 0, WallDirection::EAST));

    assert!(!vis.visited_neighbour(5, 4, WallDirection::NORTH));
    assert!(!vis.visited_neighbour(5, 4, WallDirection::EAST));
    assert!(!vis.visited_neighbour(5, 4, WallDirection::SOUTH));
    assert!(!vis.visited_neighbour(5, 4, WallDirection::WEST));

    vis.mark_visited(5, 5);
    assert!(vis.visited_neighbour(5, 4, WallDirection::NORTH));

    vis.mark_visited(5, 3);
    assert!(vis.visited_neighbour(5, 4, WallDirection::SOUTH));

    vis.mark_visited(4, 4);
    assert!(vis.visited_neighbour(5, 4, WallDirection::WEST));

    vis.mark_visited(6, 4);
    assert!(vis.visited_neighbour(5, 4, WallDirection::EAST));
}
