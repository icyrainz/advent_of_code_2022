#![allow(unused)]

use std::collections::VecDeque;

pub fn run(lines: &[String]) {
    dbg!(part1(lines));
    dbg!(part2(lines));
}

#[test]
fn test_part1() {
    assert_eq!(21, part1(&"30373
25512
65332
33549
35390"
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()));
}

fn part1(lines: &[String]) -> usize {
    let grid = read_grid(lines);
    visible_trees_count(&grid)
}

fn read_grid(lines: &[String]) -> Vec<Vec<usize>> {
    lines.iter()
        .map(|row| row
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>())
        .collect::<Vec<_>>()
}

fn visible_trees_count(grid: &[Vec<usize>]) -> usize {
    let n = grid.len();
    let m = grid[0].len();
    let mut highest_row: Vec<usize> = vec![0; n];
    let mut highest_col: Vec<usize> = vec![0; m];
    let mut visibles: Vec<Vec<bool>> = vec![vec![false; m]; n];

    for i in 0..n {
       for j in 0..m {
           if i == 0 || j == 0 || i == n-1 || j == m-1
               || grid[i][j] > highest_row[i] || grid[i][j] > highest_col[j] {
               visibles[i][j] = true;
           }
           highest_row[i] = highest_row[i].max(grid[i][j]);
           highest_col[j] = highest_col[j].max(grid[i][j]);
       }
    }

    highest_row = vec![0; n];
    highest_col = vec![0; m];

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if grid[i][j] > highest_row[i] || grid[i][j] > highest_col[j] {
                visibles[i][j] = true;
            }
            highest_row[i] = highest_row[i].max(grid[i][j]);
            highest_col[j] = highest_col[j].max(grid[i][j]);
        }
    }

    visibles.iter().map(| row| row.iter().filter(|&&x| x).count()).sum()
}

#[test]
fn test_part2() {
    assert_eq!(8, part2(&"30373
25512
65332
33549
35390"
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()));
}

fn part2(lines: &[String]) -> usize {
    let grid = read_grid(lines);
    scenic_score(&grid)
}

fn scenic_score(grid: &[Vec<usize>]) -> usize {
    let n = grid.len();
    let m = grid[0].len();
    let mut scores: Vec<Vec<usize>> = vec![vec![1; m]; n];

    for i in 1..n-1 {
        for j in 1..m-1 {
            let height = grid[i][j];

            let mut k = i - 1;
            while k > 0 && grid[k][j] < height { k -= 1; }
            scores[i][j] *= i - k;
            k = j - 1;
            while k > 0 && grid[i][k] < height { k -= 1; }
            scores[i][j] *= j - k;
        }
    }
    for i in (1..n-1).rev() {
        for j in (1..m-1).rev() {
            let height = grid[i][j];

            let mut k = i + 1;
            while k < n-1 && grid[k][j] < height { k += 1; }
            scores[i][j] *= k - i;
            k = j + 1;
            while k < m-1 && grid[i][k] < height { k += 1; }
            scores[i][j] *= k - j;
        }
    }
    scores
        .iter()
        .flat_map(|row| row.iter())
        .max()
        .unwrap()
        .clone()
}