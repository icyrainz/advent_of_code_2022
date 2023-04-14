#![allow(unused)]
use std::collections::HashSet;

pub fn run(lines: Vec<String>) {
    dbg!(part1(&lines));
//    dbg!(part2(&lines));
}

#[test]
fn test_part1() {
    assert_eq!(16, part1(&vec!["vJrwpWtwJgWrhcsFMMfFFhFp".to_string()]));    
    assert_eq!(38, part1(&vec!["jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()]));    
}

pub fn part1(lines: &Vec<String>) -> u32 {
    lines.iter().map(|line| {
        let half_size = line.len() / 2;
        let (first_half, second_half) = line.as_bytes().split_at(half_size);

        let mut first_set: HashSet<u8> = HashSet::new();
        first_half.iter().for_each(|&c| {
            first_set.insert(c);
        });

        let common_item = second_half.iter().find(|&c| first_set.contains(c)).unwrap();

        if b'a' <= *common_item && *common_item <= b'z' {
            (*common_item - b'a') as u32 + 1
        } else if b'A' <= *common_item && *common_item <= b'Z' {
            (*common_item - b'A') as u32 + 1 + 26
        } else {
            panic!("Invalid char");
        }
    }).sum()
}