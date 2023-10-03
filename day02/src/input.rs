use crate::strategy_guide::{EncryptedOpponentMove, EncryptedPlayerMove, StrategyGuide};
use std::fs::File;
use std::io::{BufRead, BufReader};
use shared::open_file;

pub fn get_strategy_guides_from_input(
    input_file_path: &str,
    strategy_guides: &mut Vec<StrategyGuide>,
) {
    let buf_reader = open_file(input_file_path);

    read_lines(buf_reader, strategy_guides, add_strategy_guide);
}

fn add_strategy_guide(strategy_guides: &mut Vec<StrategyGuide>, guide: Vec<&str>) {
    let opponent_move: EncryptedOpponentMove = match guide[0] {
        "A" => EncryptedOpponentMove::A,
        "B" => EncryptedOpponentMove::B,
        "C" => EncryptedOpponentMove::C,
        _ => {
            panic!("The input contains an unknown opponent move")
        }
    };

    let player_move: EncryptedPlayerMove = match guide[1] {
        "X" => EncryptedPlayerMove::X,
        "Y" => EncryptedPlayerMove::Y,
        "Z" => EncryptedPlayerMove::Z,
        _ => {
            panic!("The input contains an unknown player move")
        }
    };

    strategy_guides.push(StrategyGuide {
        opponent_move,
        player_move,
    });
}

fn read_lines<T>(
    buf_reader: BufReader<File>,
    vec: &mut Vec<T>,
    mut handle_line: impl FnMut(&mut Vec<T>, Vec<&str>),
) {
    for line in buf_reader.lines() {
        match line {
            Ok(line) if line.trim().len() != 0 => {
                let guide: Vec<&str> = line.split(" ").collect();
                handle_line(vec, guide);
            }
            Ok(_) => {}
            Err(_) => {
                panic!("Error reading lines");
            }
        }
    }
}
