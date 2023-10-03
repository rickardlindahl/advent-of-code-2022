use crate::rock_paper_scissors::{Move, Outcome};
use crate::strategy_guide::StrategyGuide;

impl Move {
    pub fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn calculate_outcome(player_move: Move, opponent_move: Move) -> Outcome {
    if player_move.wins_over() == opponent_move {
        Outcome::Win
    } else if opponent_move.wins_over() == player_move {
        Outcome::Lose
    } else {
        Outcome::Draw
    }
}

pub fn calculate_score_part1(strategy_guide: &StrategyGuide) -> i32 {
    let outcome = calculate_outcome(
        strategy_guide.player_move.decrypt(),
        strategy_guide.opponent_move.decrypt(),
    );

    outcome.score() + strategy_guide.player_move.decrypt().score()
}

pub fn calculate_score_part2(strategy_guide: &StrategyGuide) -> i32 {
    let outcome = strategy_guide.player_move.decrypt_outcome();

    let player_move = match outcome {
        Outcome::Lose => strategy_guide.opponent_move.decrypt().wins_over(),
        Outcome::Draw => strategy_guide.opponent_move.decrypt(),
        Outcome::Win => strategy_guide.opponent_move.decrypt().loses_against(),
    };

    outcome.score() + player_move.score()
}
