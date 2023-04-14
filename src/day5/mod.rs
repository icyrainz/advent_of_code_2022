#![allow(unused)]
use std::collections::HashSet;
use std::cmp::Reverse;
use regex::Regex;

pub fn run(lines: Vec<String>) {
    dbg!(part1(&lines));
    dbg!(part2(&lines));
}

#[test]
fn test_parse() {
    let data = "        [C] [B] [H]                
[W]     [D] [J] [Q] [B]            
[P] [F] [Z] [F] [B] [L]            
[G] [Z] [N] [P] [J] [S] [V]        
[Z] [C] [H] [Z] [G] [T] [Z]     [C]
[V] [B] [M] [M] [C] [Q] [C] [G] [H]
[S] [V] [L] [D] [F] [F] [G] [L] [F]
[B] [J] [V] [L] [V] [G] [L] [N] [J]
 1   2   3   4   5   6   7   8   9 ".lines().map(|line| line.to_string()).collect();
    
    let stacks = parse_stacks(&data);
    assert_eq!(9, stacks.len());
}

fn parse_stacks(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    
    for line in lines {
        for col in 0..stacks.len() {
            let start_index = col*4 as usize;
            let mut cell = line[start_index..start_index+3].chars();
            
            match cell.next().unwrap() {
                '[' => {
                    stacks[col].insert(0, cell.next().unwrap());
                },
                _ => {}
            }
        }
    }
    
    stacks
}

fn capture_actions(actions: &Vec<String>) -> Vec<(usize,usize, usize)> {
    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    actions.iter().map(|action| {
        pattern.captures(&action).map(|captures| {
            (
                captures[1].parse::<usize>().unwrap(),
                captures[2].parse::<usize>().unwrap(),
                captures[3].parse::<usize>().unwrap(),
            )
        })
    }).flatten().collect::<Vec<_>>()
}

#[test]
fn test_move_stack1() {
    let mut stacks = vec![vec!['A', 'B', 'C'], vec!['D']];
    let actions = capture_actions(&vec!["move 2 from 1 to 2".to_string()]);

    let result = stacks.iter().map(|st| {
        match st.last() {
            Some(item) => *item,
            None => ' ',
        }
    }).collect::<String>();

    assert_eq!("AB", result);
}

fn move_stack1(stacks: &mut Vec<Vec<char>>, actions: &Vec<(usize, usize, usize)>) {
    for (count, source, target) in actions {
        for _ in 0..*count {
            let popped = stacks[source-1].pop().unwrap();
            stacks[target-1].push(popped);
        }
    }
}

#[test]
fn test_move_stack2() {
    let mut stacks = vec![vec!['A', 'B', 'C'], vec!['D']];
    let actions = capture_actions(&vec!["move 2 from 1 to 2".to_string()]);
    move_stack2(&mut stacks, &actions);
    
    let result = stacks.iter().map(|st| {
        match st.last() {
            Some(item) => *item,
            None => ' ',
        }
    }).collect::<String>();
    
    assert_eq!("AC", result);
}

fn move_stack2(stacks: &mut Vec<Vec<char>>, actions: &Vec<(usize, usize, usize)>) {
    for (count, source, target) in actions {
        let mut popped: Vec<char> = Vec::new();
        for _ in 0..*count {
            popped.push(stacks[source-1].pop().unwrap());
        }

        popped.iter().rev().for_each(|&c| stacks[target-1].push(c));
    }
}

fn common(lines: &Vec<String>, move_func: &dyn Fn(&mut Vec<Vec<char>>, &Vec<(usize,usize, usize)>)) -> String {
    let empty_line_idx = lines.iter().enumerate().find_map(|(idx, line)| {
        match line.is_empty() {
            true => Some(idx),
            false => None,
        }
    }).unwrap();

    let data_part = lines[0..empty_line_idx].to_vec();
    let mut stacks = parse_stacks(&data_part);
    let actions = capture_actions(&lines[empty_line_idx+1..].to_vec());
    
    move_func(&mut stacks, &actions);

    stacks.iter().map(|st| {
        match st.last() {
            Some(item) => *item,
            None => ' ',
        }
    }).collect::<String>()
}

fn part1(lines: &Vec<String>) -> String {
    common(lines, &move_stack1)
}

fn part2(lines: &Vec<String>) -> String {
    common(lines, &move_stack2)
}