
pub struct SquareMaze {
    cells: Vec<u8>
}

impl SquareMaze {
    pub fn new(width: usize, height: usize) -> SquareMaze {
        // TODO init Vec with 1111_1111 maze cell data!
        return SquareMaze{cells: Vec::with_capacity(width*height)}
    }

    //Only for testing!
    pub fn raw_cell0(self) -> u8 {
        return self.cells[0];
    }
}
