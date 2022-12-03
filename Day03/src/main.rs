extern crate core;

use std::fs::File;
use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead};

#[derive(Clone)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}

fn main() {
    let part_1_final_score = load_data()
        .iter()
        .enumerate()
        .map(|(i, r)| duplicate(i, r))
        .map(|c| score_char(c))
        .fold(0, |sum, ms| sum + ms);
    let part_2_final_score = load_data()
        .iter()
        .collect::<Vec<&Rucksack>>()
        .chunks(3)
        .enumerate()
        .map(|(i, r)| overlap(i, r))
        .map(|c| score_char(c))
        .fold(0, |sum, ms| sum + ms);
    println!("Part 1 final score: {}", part_1_final_score);
    println!("Part 2 final score: {}", part_2_final_score);
}

fn score_char(c: char) -> u32 {
    return match c {
        'a'..='z' => 1 + (c as u32 - 'a' as u32),
        'A'..='Z' => 27 + (c as u32 - 'A' as u32),
        _ => panic!("Invalid character '{}'", c),
    }
}

fn duplicate(line: usize, r: &Rucksack) -> char {
    let mut in_first_compartment = HashSet::new();
    r
        .compartment1
        .chars()
        .for_each(|c| { in_first_compartment.insert(c); });

    for c2 in r.compartment2.chars() {
        if in_first_compartment.contains(&c2) {
            return c2;
        }
    }

    panic!("Failed to find duplicate in rucksackk {} ({} <-> {})", line, r.compartment1, r.compartment2);
}

fn overlap(group: usize, rs: &[&Rucksack]) -> char {
    let mut counts = HashMap::new();
    for r in rs.iter() {
        let mut in_sack = HashSet::new();
        for cpt in [r.compartment1.clone(), r.compartment2.clone()] {
            for c in cpt.chars() {
                if !in_sack.contains(&c) {
                    in_sack.insert(c);
                    let count = counts.entry(c).or_insert(0);
                    *count += 1;
                }
            }
        }
    }

    for (c, count) in counts.iter() {
        if *count >= 3 {
            return *c;
        }
    }

    panic!("Could not find 3-way overlapping duplicate in group {}", group);
}

fn load_data() -> Vec<Rucksack> {
    let file = File::open("./data/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut rucksacks = Vec::new();

    for line in lines {
        if let Ok(r) = line {
            rucksacks.push(Rucksack{
                compartment1: r[..r.len()/2].to_string(),
                compartment2: r[r.len()/2..].to_string(),
            });
        }
    }

    return rucksacks;
}