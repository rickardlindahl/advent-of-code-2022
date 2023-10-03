use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn open_file(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path);

    let buf_reader = match file {
        Ok(file) => BufReader::new(file),
        Err(e) => {
            panic!("Unable to open file {} - {}", file_path, e);
        }
    };

    buf_reader
}

pub fn read_lines<F>(buf_reader: BufReader<File>, mut handle_line: F)
where
    F: FnMut(String),
{
    for line in buf_reader.lines() {
        match line {
            Ok(line) => {
                handle_line(line);
            }
            Err(_) => {
                panic!("Error reading lines");
            }
        }
    }
}
