#![allow(unused)]

use std::collections::HashSet;
use std::hint::spin_loop;

#[test]
fn part1() {
    let data = crate::data("day10");
    read_instruction(&data);
}
#[test]
fn test_read_instructions() {
    let data = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".lines().map(|x| x.to_string()).collect::<Vec<_>>();

    read_instruction(&data);
}

fn set_sprite(sprites: &mut Vec<bool>, i: i32) {
    let len = sprites.len();
    let start = sprites.iter().position(|s| *s).unwrap_or(0);
    if start < len { sprites[start] = false; }
    if start+1 < len { sprites[start+1] = false; }
    if start+2 < len { sprites[start+2] = false; }

    if 0 <= i-1 && i-1 < len as i32 { sprites[(i-1) as usize] = true; }
    if 0 <= i && i < len as i32 { sprites[i as usize] = true; }
    if 0 <= i+1 && i+1 < len as i32 { sprites[(i+1) as usize] = true; }

    // println!("{}", sprites.iter().map(|&i| if i { '#' } else { '.' }).collect::<String>());
}

fn display_crt(sprites: &Vec<bool>, cycle: i32) {
    let cycle = ((cycle-1) % 40) as usize;
    if cycle == 0 { println!(); }
    if sprites[cycle] { print!("#"); } else { print!("."); }
}

fn update(
    cycle_count: &mut i32,
    signal_strength: &mut i32,
    register_val: i32,
    sprites: &mut Vec<bool>) {

    *cycle_count += 1;

    let report_marks = (0..=5).map(|i| 20 + 40 * i).collect::<HashSet<_>>();
    if report_marks.contains(cycle_count) {
        *signal_strength += *cycle_count * register_val;
        // dbg!(cycle_count, register_val, signal_strength);
    }

    set_sprite(sprites, register_val);
    display_crt(sprites, *cycle_count);
}

fn read_instruction(commands: &[String]) -> i32 {
    let mut register_val = 1i32;
    let mut cycle_count = 0i32;
    let mut signal_strength = 0i32;

    let mut sprites: Vec<bool> = vec![false; 40];
    set_sprite(&mut sprites, register_val);

    for command in commands {
        update(&mut cycle_count, &mut signal_strength, register_val, &mut sprites);

        let command = command.split_whitespace().collect::<Vec<&str>>();
        match command[0] {
            "noop" => { },
            "addx" => {
                let add = command[1].parse::<i32>().unwrap();

                update(&mut cycle_count, &mut signal_strength, register_val, &mut sprites);

                register_val += add;
            },
            _ => { }
        }
    }

    println!();
    dbg!(signal_strength)
}