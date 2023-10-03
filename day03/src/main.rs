mod input;
use input::get_rucksacks_from_input;
mod rucksack;
use rucksack::{find_badge, Rucksack};
mod calculations;
use calculations::calculate_char_score;

fn main() {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/input.txt");
    get_rucksacks_from_input(input_file_path, &mut rucksacks);

    println!("---- Day 3 ----");
    println!("Part 1: {}", part1(&rucksacks));
    println!("Part 2: {}", part2(&rucksacks));
    println!("---------------");
}

fn part1(rucksacks: &Vec<Rucksack>) -> u32 {
    rucksacks.into_iter().fold(0_u32, |acc, rucksack| {
        acc + calculate_char_score(rucksack.find_duplicate_item().unwrap())
    })
}

fn part2(rucksacks: &Vec<Rucksack>) -> u32 {
    let groups: Vec<&[Rucksack]> = rucksacks.chunks(3).collect();

    groups.into_iter().fold(0, |acc, group| {
        acc + calculate_char_score(find_badge(&group[0], &group[1], &group[2]).unwrap())
    })
}
