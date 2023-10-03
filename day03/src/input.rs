use crate::rucksack::Rucksack;
use shared::{open_file, read_lines};

pub fn get_rucksacks_from_input(input_file_path: &str, rucksacks: &mut Vec<Rucksack>) {
    let buf_reader = open_file(input_file_path);

    read_lines(buf_reader, |line| {
        add_rucksack(rucksacks, line);
    });
}

fn add_rucksack(rucksacks: &mut Vec<Rucksack>, supplies: String) {
    rucksacks.push(Rucksack::new(supplies));
}
