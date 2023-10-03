use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::elf::Elf;

pub fn get_elves_from_input(input_file_path: &str, elves: &mut Vec<Elf>) {
    let buf_reader = shared::open_file(input_file_path);

    read_lines(buf_reader, elves, add_calories, |elves| {
        add_elf(elves, Elf::new());
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

fn read_lines<T>(
    buf_reader: BufReader<File>,
    vec: &mut Vec<T>,
    mut on_number: impl FnMut(&mut Vec<T>, u32),
    mut on_empty: impl FnMut(&mut Vec<T>),
) {
    for line in buf_reader.lines() {
        match line {
            Ok(line) if line.parse::<u32>().is_ok() => {
                on_number(vec, line.parse::<u32>().unwrap());
            }
            Ok(line) if line.trim().is_empty() => {
                on_empty(vec);
            }
            Ok(_) => {}
            Err(_) => {
                panic!("Error reading lines");
            }
        }
    }
}
