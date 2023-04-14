#![allow(unused)]

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Elf {
    food_calories: Vec<u32>,
    total_calories: u64,
}

impl Elf {
    pub fn new() -> Self {
        Elf { food_calories: Vec::new(), total_calories: 0 }
    }

    pub fn has_food(&self) -> bool {
        !self.food_calories.is_empty()
    }

    pub fn add_food(&mut self, food_calo: u32) {
        self.food_calories.push(food_calo);
        self.total_calories += food_calo as u64;
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories.cmp(&other.total_calories)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_calories.partial_cmp(&other.total_calories)
    }
}

pub fn run(lines: Vec<String>) {
    let mut elves: BinaryHeap<Elf> = BinaryHeap::new();
    let mut cur_elf: Elf = Elf::new();

    for text in lines {
        if text.is_empty() {
            if cur_elf.has_food() {
                elves.push(cur_elf.clone());
            }
            cur_elf = Elf::new()
        } else {
            let food = text.parse::<u32>().unwrap();
            cur_elf.add_food(food);
        }
    }

    if cur_elf.has_food() {
        elves.push(cur_elf.clone());
    }

    println!("Elves count: {}", elves.len());
    println!("Elf with highest calories: {:?}", elves.peek().unwrap().total_calories);
    
    let mut top_3: u64 = 0;
    for _ in 0..3 {
        top_3 += elves.pop().unwrap().total_calories;
    }
    
    println!("Elves top3 calories total: {}", top_3);
}
