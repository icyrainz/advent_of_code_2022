#![allow(unused)]

pub fn run(lines: Vec<String>) {
    dbg!(part1(&lines));
    dbg!(part2(&lines));
}

fn part1(lines: &Vec<String>) -> u32 {
    let get_point = |c| match c {
        'X' => 1, // rock
        'Y' => 2, // paper
        'Z' => 3, // scissor
        _ => panic!("Not supported action!"),
    };
    
    let compare = |(left, right): (char, char)| match (left, right) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, 
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        _ => panic!("Not supported action!"),
    };

    let mut score: u32 = 0;
    for line in lines {
        let first_action = line.chars().nth(0).unwrap();
        let second_action = line.chars().nth(2).unwrap();

        score += compare((first_action, second_action)) + get_point(second_action);
    }

    score
}

fn part2(lines: &Vec<String>) -> u32 {
    let get_match_point = |c| match c {
        'X' => 0, // lose
        'Y' => 3, // draw
        'Z' => 6, // win
        _ => panic!("Not supported action!"),
    };

    let get_choice_point = |c| match c {
        'A' => 1, // rock
        'B' => 2, // paper
        'C' => 3, // scissor
        _ => panic!("Not supported action!"),
    };

    let get_win_choice = |c| match c {
        'A' => 'B',
        'B' => 'C',
        'C' => 'A',
        _ => panic!("Not supported action!"),
    };

    let get_lose_choice = |c| match c {
        'A' => 'C',
        'B' => 'A',
        'C' => 'B',
        _ => panic!("Not supported action!"),
    };

    let compare = |(left, right): (char, char)| match (left, right) {
        (hand, 'X') => get_lose_choice(hand),
        (hand, 'Y') => hand,
        (hand, 'Z') => get_win_choice(hand),
        _ => panic!("Not supported action!"),
    };

    let mut score: u32 = 0;

    for line in lines {
        let first_action = line.chars().nth(0).unwrap();
        let second_action = line.chars().nth(2).unwrap();

        score += get_match_point(second_action) + get_choice_point(compare((first_action, second_action)));
    }

    score
}