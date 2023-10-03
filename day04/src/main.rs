use crate::input::get_section_assignments_from_input;
use crate::section_assignment::{SectionAssignmentPair};

mod input;
mod section_assignment;

fn main() {
    let mut section_assignment_pairs: Vec<SectionAssignmentPair> = Vec::new();

    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/input.txt");
    get_section_assignments_from_input(input_file_path, &mut section_assignment_pairs);

    println!("---- Day 4 ----");
    println!("Part 1: {}", part1(&section_assignment_pairs));
    println!("Part 2: {}", part2(&section_assignment_pairs));
    println!("---------------");
}

fn part1(section_assignment_pairs: &Vec<SectionAssignmentPair>) -> u32 {
    section_assignment_pairs.into_iter().fold(0, |acc, pair| {
        match pair.fully_overlaps() {
            true => acc + 1,
            false => acc
        }
    })
}

fn part2(section_assignment_pairs: &Vec<SectionAssignmentPair>) -> u32 {
    section_assignment_pairs.into_iter().fold(0, |acc, pair| {
        match pair.overlaps() {
            true => acc + 1,
            false => acc
        }
    })
}
