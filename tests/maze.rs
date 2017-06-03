extern crate maze;

use maze::maze::{to_hex_string};

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
