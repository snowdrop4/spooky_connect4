use crate::player::Player;
use crate::position::Position;
use std::fmt;

pub const STANDARD_COLS: usize = 7;
pub const STANDARD_ROWS: usize = 6;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    cells: Vec<Option<Player>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            cells: vec![None; width * height],
            width,
            height,
        }
    }

    pub fn standard() -> Self {
        Self::new(STANDARD_COLS, STANDARD_ROWS)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index(&self, col: usize, row: usize) -> usize {
        row * self.width + col
    }

    pub fn get_piece(&self, pos: &Position) -> Option<Player> {
        if pos.is_valid(self.width, self.height) {
            self.cells[pos.to_index(self.width)]
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, pos: &Position, player: Option<Player>) {
        if pos.is_valid(self.width, self.height) {
            self.cells[pos.to_index(self.width)] = player;
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![None; self.width * self.height];
    }

    pub fn is_board_full(&self) -> bool {
        // Check if top row is full
        (0..self.width).all(|col| {
            let index = self.index(col, self.height - 1);
            self.cells[index].is_some()
        })
    }

    pub fn column_height(&self, col: usize) -> usize {
        if col >= self.width {
            return 0;
        }

        for row in (0..self.height).rev() {
            let index = self.index(col, row);
            if self.cells[index].is_some() {
                return row + 1;
            }
        }
        0
    }

    pub fn is_column_full(&self, col: usize) -> bool {
        if col >= self.width {
            return true;
        }
        let index = self.index(col, self.height - 1);
        self.cells[index].is_some()
    }

    pub fn drop_piece(&mut self, col: usize, player: Player) -> Option<usize> {
        if col >= self.width || self.is_column_full(col) {
            return None;
        }

        for row in 0..self.height {
            let index = self.index(col, row);
            if self.cells[index].is_none() {
                self.cells[index] = Some(player);
                return Some(row);
            }
        }

        None
    }

    pub fn check_win(&self, pos: &Position, player: Player) -> bool {
        // Check horizontal, vertical, and both diagonals
        self.check_direction(pos, player, 1, 0)  // Horizontal -
            || self.check_direction(pos, player, 0, 1)  // Vertical |
            || self.check_direction(pos, player, 1, 1)  // Diagonal /
            || self.check_direction(pos, player, 1, -1) // Diagonal \
    }

    fn check_direction(&self, pos: &Position, player: Player, dcol: i32, drow: i32) -> bool {
        let mut count = 1; // Count the piece at pos

        // Count in positive direction
        count += self.count_in_direction(pos, player, dcol, drow);

        // Count in negative direction
        count += self.count_in_direction(pos, player, -dcol, -drow);

        count >= 4
    }

    fn count_in_direction(&self, pos: &Position, player: Player, dcol: i32, drow: i32) -> usize {
        let mut count = 0;
        let mut col = pos.col as i32 + dcol;
        let mut row = pos.row as i32 + drow;

        while col >= 0 && col < self.width as i32 && row >= 0 && row < self.height as i32 {
            let index = self.index(col as usize, row as usize);
            if let Some(p) = self.cells[index] {
                if p == player {
                    count += 1;
                    col += dcol;
                    row += drow;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        count
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::standard()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..self.height).rev() {
            write!(f, "|")?;

            for col in 0..self.width {
                let index = self.index(col, row);

                let c = if let Some(player) = self.cells[index] {
                    player.to_char()
                } else {
                    '.'
                };

                write!(f, "{}", c)?;
                write!(f, "|")?;
            }

            writeln!(f)?;
        }

        // Column numbers
        write!(f, " ")?;
        for col in 0..self.width {
            write!(f, "{} ", col)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board_creation() {
        let board = Board::new(6, 6);
        assert!(!board.is_board_full());
        for col in 0..6 {
            assert_eq!(board.column_height(col), 0);
            assert!(!board.is_column_full(col));
        }

        let board = Board::new(10, 10);
        assert!(!board.is_board_full());
        for col in 0..10 {
            assert_eq!(board.column_height(col), 0);
            assert!(!board.is_column_full(col));
        }
    }

    #[test]
    fn test_standard_board_creation() {
        let board = Board::standard();
        assert!(!board.is_board_full());
        for col in 0..STANDARD_COLS {
            assert_eq!(board.column_height(col), 0);
            assert!(!board.is_column_full(col));
        }
    }

    #[test]
    fn test_drop_piece() {
        let mut board = Board::standard();
        let row = board.drop_piece(0, Player::Red);
        assert_eq!(row, Some(0));

        let pos = Position::new(0, 0);
        assert_eq!(board.get_piece(&pos), Some(Player::Red));
    }

    #[test]
    fn test_drop_multiple_pieces() {
        let mut board = Board::standard();

        // Drop 3 pieces in column 0
        assert_eq!(board.drop_piece(0, Player::Red), Some(0));
        assert_eq!(board.drop_piece(0, Player::Yellow), Some(1));
        assert_eq!(board.drop_piece(0, Player::Red), Some(2));

        assert_eq!(board.column_height(0), 3);
    }

    #[test]
    fn test_column_full() {
        let mut board = Board::standard();

        // Fill column 0
        for _ in 0..STANDARD_ROWS {
            board.drop_piece(0, Player::Red);
        }

        assert!(board.is_column_full(0));
        assert_eq!(board.drop_piece(0, Player::Red), None);
    }

    #[test]
    fn test_board_full() {
        let mut board = Board::standard();

        // Fill entire board
        for col in 0..STANDARD_COLS {
            for _ in 0..STANDARD_ROWS {
                board.drop_piece(col, Player::Red);
            }
        }

        assert!(board.is_board_full());
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = Board::standard();

        // Place 4 red pieces horizontally
        for col in 0..4 {
            board.drop_piece(col, Player::Red);
        }

        let pos = Position::new(3, 0);
        assert!(board.check_win(&pos, Player::Red));
    }

    #[test]
    fn test_vertical_win() {
        let mut board = Board::standard();

        // Place 4 red pieces vertically in column 0
        for _ in 0..4 {
            board.drop_piece(0, Player::Red);
        }

        let pos = Position::new(0, 3);
        assert!(board.check_win(&pos, Player::Red));
    }

    #[test]
    fn test_diagonal_win_ascending() {
        let mut board = Board::standard();

        // Create diagonal win (/)
        // Col 0: R
        board.drop_piece(0, Player::Red);

        // Col 1: Y, R
        board.drop_piece(1, Player::Yellow);
        board.drop_piece(1, Player::Red);

        // Col 2: Y, Y, R
        board.drop_piece(2, Player::Yellow);
        board.drop_piece(2, Player::Yellow);
        board.drop_piece(2, Player::Red);

        // Col 3: Y, Y, Y, R
        board.drop_piece(3, Player::Yellow);
        board.drop_piece(3, Player::Yellow);
        board.drop_piece(3, Player::Yellow);
        board.drop_piece(3, Player::Red);

        let pos = Position::new(3, 3);
        assert!(board.check_win(&pos, Player::Red));
    }

    #[test]
    fn test_diagonal_win_descending() {
        let mut board = Board::standard();

        // Create diagonal win (\)
        // Col 0: Y, Y, Y, R
        board.drop_piece(0, Player::Yellow);
        board.drop_piece(0, Player::Yellow);
        board.drop_piece(0, Player::Yellow);
        board.drop_piece(0, Player::Red);

        // Col 1: Y, Y, R
        board.drop_piece(1, Player::Yellow);
        board.drop_piece(1, Player::Yellow);
        board.drop_piece(1, Player::Red);

        // Col 2: Y, R
        board.drop_piece(2, Player::Yellow);
        board.drop_piece(2, Player::Red);

        // Col 3: R
        board.drop_piece(3, Player::Red);

        let pos = Position::new(3, 0);
        assert!(board.check_win(&pos, Player::Red));
    }

    #[test]
    fn test_no_win() {
        let mut board = Board::standard();

        // Place 3 pieces (not enough for win)
        for col in 0..3 {
            board.drop_piece(col, Player::Red);
        }

        let pos = Position::new(2, 0);
        assert!(!board.check_win(&pos, Player::Red));
    }

    #[test]
    fn test_get_set() {
        let mut board = Board::standard();
        let pos = Position::new(3, 2);

        assert_eq!(board.get_piece(&pos), None);

        board.set_piece(&pos, Some(Player::Red));
        assert_eq!(board.get_piece(&pos), Some(Player::Red));

        board.set_piece(&pos, None);
        assert_eq!(board.get_piece(&pos), None);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut board = Board::standard();

        // Out of bounds position
        let pos = Position::new(10, 10);
        assert_eq!(board.get_piece(&pos), None);

        board.set_piece(&pos, Some(Player::Red));
        assert_eq!(board.get_piece(&pos), None);
    }

    #[test]
    fn test_invalid_column() {
        let mut board = Board::standard();

        assert_eq!(board.drop_piece(10, Player::Red), None);
        assert_eq!(board.column_height(10), 0);
        assert!(board.is_column_full(10));
    }
}
