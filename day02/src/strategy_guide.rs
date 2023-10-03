use crate::rock_paper_scissors::{Move, Outcome};

pub struct StrategyGuide {
    pub opponent_move: EncryptedOpponentMove,
    pub player_move: EncryptedPlayerMove,
}

pub enum EncryptedOpponentMove {
    A,
    B,
    C,
}

impl EncryptedOpponentMove {
    pub fn decrypt(&self) -> Move {
        match self {
            EncryptedOpponentMove::A => Move::Rock,
            EncryptedOpponentMove::B => Move::Paper,
            EncryptedOpponentMove::C => Move::Scissors,
        }
    }
}

pub enum EncryptedPlayerMove {
    X,
    Y,
    Z,
}

impl EncryptedPlayerMove {
    pub fn decrypt(&self) -> Move {
        match self {
            EncryptedPlayerMove::X => Move::Rock,
            EncryptedPlayerMove::Y => Move::Paper,
            EncryptedPlayerMove::Z => Move::Scissors,
        }
    }
    pub fn decrypt_outcome(&self) -> Outcome {
        match self {
            EncryptedPlayerMove::X => Outcome::Lose,
            EncryptedPlayerMove::Y => Outcome::Draw,
            EncryptedPlayerMove::Z => Outcome::Win,
        }
    }
}
