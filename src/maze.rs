//some general maze functionalities and definitions

pub fn to_hex_string(seed: [u32; 4]) -> String {
    return format!("{:08X}-{:08X}-{:08X}-{:08X}", seed[0], seed[1], seed[2], seed[3]);
}

pub struct MetaData {
    pub seed: String,
    pub dead_ends: u32
}

impl MetaData {
    pub fn new_empty() -> MetaData {
        return MetaData{seed: "".to_string(), dead_ends: 0};
    }
}
