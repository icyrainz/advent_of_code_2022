#![allow(unused)]

use std::collections::{HashMap, HashSet};

pub fn run(lines: Vec<String>) {
    part1(&lines);
    part2(&lines);
}

#[test]
fn test_part1() {
    assert_eq!(5, read_signal("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(11, read_signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
}

fn part1(lines: &Vec<String>) {
    let result = read_signal(&lines[0], 4);
    dbg!(result);
}

fn part2(lines: &Vec<String>) {
    let result = read_signal(&lines[0], 14);
    dbg!(result);
}

fn read_signal(data: &str, window_size: usize) -> usize {
    data.as_bytes().windows(window_size)
        .position(|window|
            window.to_vec().into_iter().collect::<HashSet<u8>>().len() == window_size)
        .map(|i| i + window_size).unwrap()
}