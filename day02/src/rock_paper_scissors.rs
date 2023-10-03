#[derive(PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn wins_over(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
    pub fn loses_against(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

pub enum Outcome {
    Draw,
    Win,
    Lose,
}
