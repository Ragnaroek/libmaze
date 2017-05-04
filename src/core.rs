
pub struct SquareMaze {
    cells: Vec<u8>
}

impl SquareMaze {
    pub fn new(width: usize, height: usize) -> SquareMaze {
        let size = width*height;
        let mut cells = Vec::with_capacity(size);
        for i in 0..size {
            cells.insert(i, 255);
        }
        return SquareMaze{cells: cells};
    }

    // TODO Impl generation algorithm (recursive!)

    //Only for testing!
    pub fn raw_cell0(self) -> u8 {
        return self.cells[0];
    }
}
