// src/main.rs
use std::env;
use file_reader::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_val = &args[1..].join("");
    println!("the path is {}", path_val);
    let lines = read_file(path_val);
    println!("the lines are {:#?}", lines);
}