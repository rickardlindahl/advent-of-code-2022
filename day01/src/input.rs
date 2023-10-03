use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn open_file(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path);

    let buf_reader = match file {
        Ok(file) => {
            BufReader::new(file)
        }
        Err(e) => {
            panic!("Unable to open file {} - {}", file_path, e);
        }
    };

    buf_reader
}

pub fn read_lines<T>(buf_reader: BufReader<File>, vec: &mut Vec<T>, mut on_number:impl FnMut(&mut Vec<T>, u32), mut on_empty: impl FnMut(&mut Vec<T>)) {
    for line in buf_reader.lines() {
        match line {
            Ok(line) if line.parse::<u32>().is_ok() => {
                on_number(vec, line.parse::<u32>().unwrap());
            }
            Ok(line) if line.trim().is_empty() => {
                on_empty(vec);
            }
            Ok(_) => {}
            Err(_) => { panic!("Error reading lines"); }
        }
    }
}
