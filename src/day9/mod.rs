#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter, write};

#[test]
fn day9() {
    let lines = crate::data("day9");

    dbg!(part1(&lines));
    dbg!(part2(&lines));
}

#[derive(Copy, Clone, Hash)]
struct Cell {
    x: i32,
    y: i32,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x: {}; y: {}]", self.x, self.y)
    }
}

impl Cell {
    fn jump(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }

    fn get_dist(&self, other: &Cell) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    fn is_touching(&self, other: &Cell) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn pull(&self, other: Cell) -> Cell {
        if self.is_touching(&other) { return other.clone(); }

        let directions =
            vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
        directions.iter().map(|(dx, dy)| {
            let new_cell = Cell { x: other.x + dx, y: other.y + dy };
            (new_cell, self.get_dist(&new_cell))
        }).min_by_key(|(_, dis)| *dis).unwrap().0
    }
}

fn move_rope(actions: &[String], tail_count: usize) -> u32 {
    let mut knots = vec![Cell { x: 0, y: 0 }; tail_count + 1];

    let mut visit: HashSet<(i32, i32)> = HashSet::new();
    visit.insert((0, 0));

    let n = 50;
    visual(n, &knots);

    for action in actions {

        let mut action = action.split( ' ');
        let direction = action.next().unwrap().chars().next().unwrap();
        let count = action.next().unwrap().parse::<u32>().unwrap();

        let (dx, dy) = match direction {
            'R' => { (0,1) },
            'L' => { (0,-1) },
            'D' => { (1,0) },
            'U' => { (-1,0) },
            _ => panic!("Invalid direction"),
        };

        for _ in 0..count {
            knots[0].jump(dx, dy);
            for i in 1..=tail_count {
                knots[i] = knots[i-1].pull(knots[i]);
            }
            visit.insert((knots[tail_count].x, knots[tail_count].y));
        }
        // visual(n, &knots);
    }

    visual_visit(n, &visit);
    visit.len() as u32
}

fn display(n: u32, set: &HashMap<(u32, u32), String>) {
    for i in 0..n {
        for j in 0..n {
            let c: String = set.get(&(i as u32, j as u32)).unwrap_or(&String::from(".")).clone();
            print!("{c}");
        }
        println!();
    }
    println!();
}

fn visual(n: u32, knots: &Vec<Cell>) {

    let mut set: HashMap<(u32, u32), String> = HashMap::new();
    for i in (0.. knots.len()).rev() {
        let c = if i == 0 { "H".to_string() } else { i.to_string() };
        let x = ((n/2) as i32 + knots[i].x) as u32;
        let y = ((n/2) as i32 + knots[i].y) as u32;
        set.insert((x, y), c);
    }

    display(n, &set);
}

fn visual_visit(n: u32, visit: &HashSet<(i32, i32)>) {

    let mut set: HashMap<(u32, u32), String> = HashMap::new();
    for (x, y) in visit {
        let x = ((n/2) as i32 + x) as u32;
        let y = ((n/2) as i32 + y) as u32;
        set.insert((x, y), "x".to_string());
    }

    display(n, &set);
}

#[test]
fn test_move_ropes() {
    let data = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2".lines().map(|x| x.to_string()).collect::<Vec<_>>();

    assert_eq!(13, move_rope(&data, 1));
}

#[test]
fn test_move_ropes_9tails() {
    let data = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20".lines().map(|x| x.to_string()).collect::<Vec<_>>();

    assert_eq!(36, move_rope(&data, 9));
}

fn part1(lines: &[String]) -> u32 {
    move_rope(lines, 1)
}

fn part2(lines: &[String]) -> u32 {
    move_rope(lines, 9)
}
