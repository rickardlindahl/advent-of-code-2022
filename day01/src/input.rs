use shared::{open_file, read_lines};

use crate::elf::Elf;

pub fn get_elves_from_input(input_file_path: &str, elves: &mut Vec<Elf>) {
    let buf_reader = open_file(input_file_path);

    read_lines(buf_reader, |line| {
        if line.trim().is_empty() {
            add_elf(elves, Elf::new());
        } else {
            let calories = match line.parse::<u32>() {
                Ok(calories) => calories,
                _err => panic!(
                    "Line is not empty and not a number. Bad input! Line: {}",
                    line
                ),
            };
            add_calories(elves, calories);
        }
    });
}

fn add_calories(elves: &mut Vec<Elf>, calories: u32) {
    match elves.last_mut() {
        Some(elf) => {
            elf.add_calories(calories);
        }
        None => {
            let mut elf = Elf::new();
            elf.add_calories(calories);
            add_elf(elves, elf);
        }
    };
}

fn add_elf(elves: &mut Vec<Elf>, elf: Elf) {
    elves.push(elf);
}
