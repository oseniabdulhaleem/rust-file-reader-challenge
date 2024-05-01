use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;



pub fn read_file(path:&str) -> Vec<String> {
    let path= Path::new(path);
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        match line{
            Ok(line) => lines.push(line),
            Err(error)=> {
                panic!("Error reading line: {}",error)
            }
        }
    }

    lines
}

// I need a test for the code above