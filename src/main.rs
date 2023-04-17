#![allow(unused)]
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() { }


fn data(path: &str) -> Vec<String> {
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