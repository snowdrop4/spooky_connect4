#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub fn new(col: usize, row: usize) -> Self {
        Position { col, row }
    }

    pub fn from_index(index: usize, width: usize) -> Self {
        Position {
            col: index % width,
            row: index / width,
        }
    }

    pub fn to_index(&self, width: usize) -> usize {
        self.row * width + self.col
    }

    pub fn is_valid(&self, width: usize, height: usize) -> bool {
        self.col < width && self.row < height
    }
}
