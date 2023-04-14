#![allow(unused)]
use std::collections::HashSet;

pub fn run(lines: Vec<String>) {
    dbg!(part1(&lines));
    dbg!(part2(&lines));
}

#[test]
fn test_part1() {
    assert_eq!(16, part1(&vec!["vJrwpWtwJgWrhcsFMMfFFhFp".to_string()]));    
    assert_eq!(38, part1(&vec!["jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()]));    
}

#[test]
fn test_part2_singlegroup() {
    assert_eq!(18,
        part2(&vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ]));
}

#[test]
fn test_part2_multigroups() {
    assert_eq!(70,
        part2(&vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ]));
}

fn part1(lines: &Vec<String>) -> u32 {
    lines.iter().map(|line| {
        let half_size = line.len() / 2;
        let (first_half, second_half) = line.as_bytes().split_at(half_size);

        let first_set = first_half.iter().copied().collect::<HashSet<u8>>();

        let common_item = second_half.iter().find(|&c| first_set.contains(c)).unwrap();

        match common_item {
            b'a'..=b'z' => (common_item - b'a') as u32 + 1,
            b'A'..=b'Z' => (common_item - b'A') as u32 + 1 + 26,
            _ => panic!("Invalid char"),
        }
    }).sum()
}

fn part2(lines: &Vec<String>) -> u32 {
    let sum = lines.chunks(3).fold(0, |acc, chunk| {
        let common_item = chunk.iter()
            .map(|bag| bag.as_bytes().iter().copied().collect::<HashSet<u8>>())
            .fold(None, |acc: Option<HashSet<u8>>, set| {
                match acc {
                    Some(acc) => Some(acc.intersection(&set).copied().collect()),
                    None => Some(set),
                }
            }).unwrap().iter().next().unwrap().clone();

        let new_val = match common_item {
            b'a'..=b'z' => (common_item - b'a') as u32 + 1,
            b'A'..=b'Z' => (common_item - b'A') as u32 + 1 + 26,
            _ => panic!("Invalid char"),
        };

        acc + new_val
    });

    sum
}