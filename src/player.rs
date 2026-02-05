#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Player {
    Red = 1,
    Yellow = -1,
}

impl Player {
    pub fn opposite(&self) -> Player {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Player::Red => 'R',
            Player::Yellow => 'Y',
        }
    }

    pub fn from_char(c: char) -> Option<Player> {
        match c {
            'R' | 'r' => Some(Player::Red),
            'Y' | 'y' => Some(Player::Yellow),
            _ => None,
        }
    }

    pub fn from_int(i: i8) -> Option<Player> {
        match i {
            1 => Some(Player::Red),
            -1 => Some(Player::Yellow),
            _ => None,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player_str = match self {
            Player::Red => "Red",
            Player::Yellow => "Yellow",
        };
        write!(f, "{}", player_str)
    }
}
