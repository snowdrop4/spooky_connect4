use crate::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    pub col: usize,
    pub row: usize,
}

impl Move {
    pub fn new(col: usize, row: usize) -> Self {
        Move { col, row }
    }

    pub fn position(&self) -> Position {
        Position::new(self.col, self.row)
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move(col: {}, row: {})", self.col, self.row)
    }
}
