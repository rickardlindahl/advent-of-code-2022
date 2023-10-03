use std::fs::File;
use std::io::{BufReader};

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
