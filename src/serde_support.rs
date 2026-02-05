use crate::game::Game;
use crate::r#move::Move;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize Game as move history (column numbers)
/// Format: "3,4,2,5" or "1,2,3" where numbers are columns
impl Serialize for Game {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as move history (list of columns)
        let moves: Vec<String> = self
            .move_history()
            .iter()
            .map(|m| m.col.to_string())
            .collect();
        let moves_str = moves.join(",");

        // Always start with Red (standard), so we just serialize the moves
        serializer.serialize_str(&moves_str)
    }
}

/// Deserialize Game from move history
impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let moves_str = String::deserialize(deserializer)?;

        let mut game = Game::standard();

        if moves_str.is_empty() {
            return Ok(game);
        }

        // Parse move history and replay
        for col_str in moves_str.split(',') {
            let col: usize = col_str
                .trim()
                .parse()
                .map_err(|e| serde::de::Error::custom(format!("Invalid column number: {}", e)))?;

            let row = game.board().column_height(col);
            let mv = Move::new(col, row);

            if !game.make_move(&mv) {
                return Err(serde::de::Error::custom(format!(
                    "Invalid move: col {}",
                    col
                )));
            }
        }

        Ok(game)
    }
}

/// Serialize Move as column number
impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.col as u64)
    }
}

/// Deserialize Move from column number
impl<'de> Deserialize<'de> for Move {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let col = u64::deserialize(deserializer)? as usize;

        if col >= crate::board::STANDARD_COLS {
            return Err(serde::de::Error::custom(format!(
                "Column {} out of range",
                col
            )));
        }

        // Row will be determined when the move is made
        // For serialization purposes, we use row 0 as a placeholder
        Ok(Move::new(col, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_serde_empty() {
        let game = Game::standard();

        // Serialize to JSON
        let json = serde_json::to_string(&game).unwrap();
        assert_eq!(json, r#""""#); // Empty string for new game

        // Deserialize back
        let game2: Game = serde_json::from_str(&json).unwrap();
        assert_eq!(game2.move_history().len(), 0);
        assert!(!game2.is_over());
    }

    #[test]
    fn test_game_serde_with_moves() {
        let mut game = Game::standard();

        // Make some moves
        game.make_move(&Move::new(3, 0));
        game.make_move(&Move::new(3, 1));
        game.make_move(&Move::new(4, 0));
        game.make_move(&Move::new(4, 1));

        let moves_before = game.move_history().to_vec();

        // Serialize to JSON
        let json = serde_json::to_string(&game).unwrap();
        assert_eq!(json, r#""3,3,4,4""#);

        // Deserialize back
        let game2: Game = serde_json::from_str(&json).unwrap();

        assert_eq!(game2.move_history().len(), moves_before.len());
        for (i, mv) in game2.move_history().iter().enumerate() {
            assert_eq!(mv.col, moves_before[i].col);
        }
    }

    #[test]
    fn test_move_serde() {
        let move_ = Move::new(3, 2);

        // Serialize to JSON
        let json = serde_json::to_string(&move_).unwrap();
        assert_eq!(json, "3");

        // Deserialize back
        let move2: Move = serde_json::from_str(&json).unwrap();
        assert_eq!(move2.col, 3);
    }

    #[test]
    fn test_game_roundtrip() {
        let mut game = Game::standard();

        // Make a sequence of moves
        let moves_cols = vec![3, 3, 4, 4, 5, 5, 6];
        for col in moves_cols {
            let row = game.board().column_height(col);
            game.make_move(&Move::new(col, row));
        }

        // Serialize
        let json = serde_json::to_string(&game).unwrap();

        // Deserialize
        let game2: Game = serde_json::from_str(&json).unwrap();

        // Check they're the same
        assert_eq!(game.move_history().len(), game2.move_history().len());
        assert_eq!(game.is_over(), game2.is_over());
        assert_eq!(game.outcome(), game2.outcome());
    }

    #[test]
    fn test_bincode_game() {
        let mut game = Game::standard();
        game.make_move(&Move::new(3, 0));
        game.make_move(&Move::new(4, 0));

        // Serialize with bincode
        let encoded = bincode::serialize(&game).unwrap();

        // Deserialize
        let game2: Game = bincode::deserialize(&encoded).unwrap();

        assert_eq!(game.move_history().len(), game2.move_history().len());
        assert_eq!(game.move_history()[0].col, game2.move_history()[0].col);
    }

    #[test]
    fn test_bincode_move() {
        let move_ = Move::new(5, 2);

        // Serialize with bincode
        let encoded = bincode::serialize(&move_).unwrap();

        // Deserialize
        let move2: Move = bincode::deserialize(&encoded).unwrap();

        assert_eq!(move2.col, 5);
    }
}
