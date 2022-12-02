use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut elves = vec![0i32];
    let file = File::open("./data/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(calories) = line {
            if calories.chars().count() == 0 {
                elves.push(0);
            } else {
                let cals: i32 = calories.parse().unwrap();
                let current = elves.last_mut().unwrap();
                *current += cals;
            }
        }
    }

    // First problem: linear runtime
    let mut max = 0;
    for elf in elves.to_vec() {
        if max < elf {
            max = elf;
        }
    }

    println!("Top elf is carrying: {}", max);

    // Second problem, could technical do in linear time, but why not nLog(n) 🤷
    let mut sorted = elves.to_vec();
    sorted.sort();
    println!("Top 3 elves are carrying: {}", sorted.pop().unwrap() + sorted.pop().unwrap() + sorted.pop().unwrap());
}