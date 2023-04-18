#![allow(unused)]

use std::collections::{HashSet, VecDeque};

#[test]
fn test() {
    let map = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi".lines().map(|x| x.to_string()).collect::<Vec<String>>();

    assert_eq!(31, part1(&map));
    assert_eq!(29, part2(&map));
}

#[test]
fn test_part1() {
    let data = crate::data("day12");
    part1(&data);
}

fn part1(lines: &[String]) -> usize {
    let (mut start, mut end) = ((0,0), (0,0));
    for ( x, row  )in lines.iter().enumerate(){
        for ( y, cell ) in row.chars().enumerate() {
            match cell {
                'S' => start = (x, y),
                'E' => end = (x, y),
                _ => { },
            }
        }
    }

    dbg!(find_end(&lines, start, end))
}

#[test]
fn test_part2() {
    let data = crate::data("day12");
    part2(&data);
}
fn part2(lines: &[String]) -> usize {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut end = (0,0);

    for ( x, row  )in lines.iter().enumerate(){
        for ( y, cell ) in row.chars().enumerate() {
            match cell {
                'S' | 'a' => starts.push( (x, y)),
                'E' => end = (x, y),
                _ => { },
            }
        }
    }

    dbg!(starts.iter().map(|start| find_end(&lines, *start, end)).min().unwrap())
}
fn find_end(lines: &[String], start: (usize, usize), end: (usize, usize)) -> usize {

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);

    let mut move_count = 0;
    let n = lines.len();
    let m = lines[0].len();

    let mut cost: Vec<Vec<usize>> = vec![vec![usize::MAX; m]; n];
    cost[start.0][start.1] = 0;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == end { break; }

        let directions = vec![(0,1), (1,0), (0,-1), (-1,0)];
        for (dx, dy) in directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if 0 <= new_x && new_x < n as isize && 0 <= new_y && new_y < m as isize {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                let get_cell = |x: usize, y: usize| {
                    match lines[x].as_bytes()[y] {
                        b'S' => b'a',
                        b'E' => b'z',
                        other => other,
                    }
                };

                if get_cell(new_x, new_y) <= get_cell(x, y) + 1
                    && cost[new_x][new_y] > cost[x][y] + 1 {
                    cost[new_x][new_y] = cost[new_x][new_y].min(cost[x][y] + 1);
                    queue.push_back((new_x, new_y));
                }
            }
        }

    }

    let debug = false;
    if debug {
        cost.iter().for_each(|row| {
            row.iter().for_each(|item| {
                if *item == usize::MAX {
                    print!("{:>4}", "x");
                } else {
                    print!("{:>4}", item);
                }
            });
            println!();
        });

        println!();
    }

    cost[end.0][end.1]
}