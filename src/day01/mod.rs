use std::{fs};
use std::io::{BufRead, BufReader};

struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn new() -> Self {
        Elf { calories: Vec::new() }
    }

    fn add_calories(&mut self, calories: u32) {
        self.calories.push(calories);
    }

    fn total_calories(&self) -> u32 {
        self.calories.clone().into_iter().reduce(|acc, e| acc + e).unwrap_or(0_u32)
    }
}

fn get_elves() -> Vec<Elf> {
    let file_path = "./src/day01/input.txt";
    let file = fs::File::open(file_path).unwrap();
    let br = BufReader::new(file);

    let mut elves: Vec<Elf> = Vec::new();

    for line in br.lines() {
        match line {
            Ok(line) if line.parse::<u32>().is_ok() => {
                if let Some(elf) = elves.last_mut() {
                    elf.add_calories(line.parse::<u32>().unwrap());
                }
            }
            Ok(line) if line.trim().is_empty() => {
                elves.push(Elf::new());
            }
            Ok(_) => {}
            Err(_) => { panic!("Error reading lines"); }
        }
    }
    elves
}

pub fn part1() -> u32 {
    let elves = get_elves();

    let most_calories = elves.into_iter().fold(0, |acc, elf| {
        let elf_calories = elf.total_calories();
        if elf_calories > acc { elf_calories } else { acc }
    });

    most_calories
}


pub fn part2() -> u32 {
    let elves = get_elves();

    let top_three = elves.into_iter().fold((0_u32, 0_u32, 0_u32), |acc, elf| {
        let elf_calories = elf.total_calories();

        let (first, second, third) = acc;
        if elf_calories > first {
            (elf_calories, first, second)
        } else if elf_calories > second {
            (first, elf_calories, second)
        } else if elf_calories > third {
            (first, second, elf_calories)
        } else {
            acc
        }
    });

    let (first, second, third) = top_three;

    first + second + third
}