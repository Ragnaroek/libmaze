//some general maze functionalities and definitions

pub fn to_hex_string(seed: [u32; 4]) -> String {
    return format!("{:08X}-{:08X}-{:08X}-{:08X}", seed[0], seed[1], seed[2], seed[3]);
}
