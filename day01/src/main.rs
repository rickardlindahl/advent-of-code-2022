mod elf;
mod input;

use elf::Elf;

fn main() {
    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/input.txt");
    let mut elves: Vec<Elf> = Vec::new();
    input::get_elves_from_input(input_file_path, &mut elves);
    println!("---- Day 1 ----");
    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
    println!("---------------");
}

pub fn part1(elves: &Vec<Elf>) -> u32 {
    let most_calories = elves.into_iter().fold(0, |acc, elf| {
        if elf.calories > acc {
            elf.calories
        } else {
            acc
        }
    });

    most_calories
}

pub fn part2(elves: &Vec<Elf>) -> u32 {
    let (first, second, third) = elves.into_iter().fold((0_u32, 0_u32, 0_u32), |acc, elf| {
        let (first, second, third) = acc;
        if elf.calories > first {
            (elf.calories, first, second)
        } else if elf.calories > second {
            (first, elf.calories, second)
        } else if elf.calories > third {
            (first, second, elf.calories)
        } else {
            acc
        }
    });

    first + second + third
}
