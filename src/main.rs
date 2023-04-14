use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;
mod day2;

fn main() {
//    day1::run(read_file("day1"));
    day2::run(read_file("day2"));
}

fn read_file(path: &str) -> Vec<String> {
    let path = String::from("input/") + &path;

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut res: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(text) => {
                res.push(text);
            },
            Err(_) => panic!("Unable to read file"),
        }
    }

    res
}