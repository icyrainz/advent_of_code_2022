#![allow(unused)]
use std::collections::HashSet;
use std::cmp::Reverse;

pub fn run(lines: Vec<String>) {
    dbg!(part1(&lines));
    dbg!(part2(&lines));
}

#[test]
fn test_part1() {
    let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".lines()
    .map(|line| line.to_string())
    .collect::<Vec<String>>();
    
    assert_eq!(2, part1(&data));
}

#[test]
fn test_part2() {
    let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".lines()
    .map(|line| line.to_string())
    .collect::<Vec<String>>();

    assert_eq!(4, part2(&data));
}

fn common(lines: &Vec<String>, compare_func: &dyn Fn((u8, u8), (u8, u8)) -> bool) -> u32 {
    lines.iter()
    .filter(|line| {
        let mut sections = line.split(',')
        .map(|sec| {
            let items = sec.split('-').collect::<Vec<&str>>();
            (items[0].parse::<u8>().unwrap(), items[1].parse::<u8>().unwrap())
        }).collect::<Vec<(u8, u8)>>();

        sections.sort_by_key(|(i1, i2)| (*i1, Reverse(*i2)));
        compare_func(sections[0], sections[1])
    }).count() as u32
}

fn part1(lines: &Vec<String>) -> u32 {
    common(lines, &|(a1, a2), (b1, b2)| a1 <= b1 && a2 >= b2)
}

fn part2(lines: &Vec<String>) -> u32 {
    common(lines, &|(_, a2), (b1, _)| b1 <= a2)
}