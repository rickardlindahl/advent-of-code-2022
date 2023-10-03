mod input;
mod strategy_guide;
use strategy_guide::StrategyGuide;

mod calculations;
mod rock_paper_scissors;

pub fn part1(strategy_guides: &Vec<StrategyGuide>) -> i32 {
    strategy_guides.iter().fold(0, |acc, strategy_guide| {
        acc + calculations::calculate_score_part1(strategy_guide)
    })
}

pub fn part2(strategy_guides: &Vec<StrategyGuide>) -> i32 {
    strategy_guides.iter().fold(0, |acc, strategy_guide| {
        acc + calculations::calculate_score_part2(strategy_guide)
    })
}

fn main() {
    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/input.txt");
    let mut strategy_guides: Vec<StrategyGuide> = Vec::new();
    input::get_strategy_guides_from_input(input_file_path, &mut strategy_guides);
    println!("---- Day 2 ----");
    println!("Part 1: {}", part1(&strategy_guides));
    println!("Part 2: {}", part2(&strategy_guides));
    println!("---------------");
}
