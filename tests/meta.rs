extern crate maze;

use maze::square_maze::{SquareMaze, WallDirection};
use maze::meta::{is_dead_end, to_hex_string, Distance};

#[test]
fn should_convert_all_zero_array_to_hex() {
    let result = to_hex_string([0,0,0,0]);
    assert_eq!(result, "00000000-00000000-00000000-00000000");
}

#[test]
fn should_convert_max_u32_to_all_f_hex() {
    let max_u32 = u32::max_value();
    let result = to_hex_string([max_u32,max_u32,max_u32,max_u32]);
    assert_eq!(result, "FFFFFFFF-FFFFFFFF-FFFFFFFF-FFFFFFFF");
}

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

#[test]
fn should_init_distance_struct_with_all_zeros() {
    let dist = Distance::new_empty(10, 10);
    for x in 0..10 {
        for y in 0..10 {
            assert_eq!(dist.distance(x, y), 0);
        }
    }
}

#[test]
fn should_set_distance() {
    let mut dist = Distance::new_empty(10, 10);
    assert_eq!(dist.distance(0,0), 0);
    dist.set(0,0, 666);
    assert_eq!(dist.distance(0,0), 666);

    dist.set(9,9, 42);
    assert_eq!(dist.distance(9, 9), 42);
}
