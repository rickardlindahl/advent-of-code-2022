mod elf;
mod input;

use elf::Elf;

pub fn part1(elves: &Vec<Elf>) -> u32 {
    let most_calories = elves.into_iter().fold(0, |acc, elf| {
        let elf_calories = elf.total_calories();
        if elf_calories > acc {
            elf_calories
        } else {
            acc
        }
    });

    most_calories
}

pub fn part2(elves: &Vec<Elf>) -> u32 {
    let (first, second, third) = elves.into_iter().fold((0_u32, 0_u32, 0_u32), |acc, elf| {
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

    first + second + third
}

fn add_calories(elves: &mut Vec<Elf>, calories: u32) {
    match elves.last_mut() {
        Some(elf) => { elf.add_calories(calories); },
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

fn get_elves_from_input(input_file_path: &str, elves: &mut Vec<Elf>) {
    elves.push(Elf::new());

    let buf_reader = input::open_file(input_file_path);

    input::read_lines(buf_reader, elves, add_calories, |elves| { add_elf(elves, Elf::new()); });
}

fn main() {
    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/input.txt");
    let mut elves: Vec<Elf> = Vec::new();
    get_elves_from_input(input_file_path, &mut elves);
    println!("---- Day 1 ----");
    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
    println!("---------------");
}
