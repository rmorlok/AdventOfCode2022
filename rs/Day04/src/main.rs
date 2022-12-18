extern crate core;

use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{RangeInclusive};

#[derive(Clone)]
struct ElfPair {
    elf1: RangeInclusive<i32>,
    elf2: RangeInclusive<i32>,
}

fn main() {
    let part_1_count = load_data()
        .iter()
        .filter(|ep| elves_full_overlap(ep))
        .count();
    let part_2_count = load_data()
        .iter()
        .filter(|ep| elves_any_overlap(ep))
        .count();
    println!("Part 1 final count: {}", part_1_count);
    println!("Part 2 final count: {}", part_2_count);
}

fn elves_full_overlap(elf_pair: &ElfPair) -> bool {
    return full_overlap(elf_pair.elf1.clone(), elf_pair.elf2.clone());
}

fn full_overlap(r1: RangeInclusive<i32>, r2: RangeInclusive<i32>) -> bool {
    return (*r1.start() <= *r2.start() && *r2.end() <= *r1.end()) ||
        (*r2.start() <= *r1.start() && *r1.end() <= *r2.end());
}

fn elves_any_overlap(elf_pair: &ElfPair) -> bool {
    return full_any_overlap(elf_pair.elf1.clone(), elf_pair.elf2.clone());
}

fn full_any_overlap(r1: RangeInclusive<i32>, r2: RangeInclusive<i32>) -> bool {
    return (*r1.start() <= *r2.start() && *r2.start() <= *r1.end()) ||
        (*r2.start() <= *r1.start() && *r1.start() <= *r2.end());
}

fn str_to_range(s: String) -> RangeInclusive<i32> {
    let parts = s.split('-').collect::<Vec<&str>>();
    let start = parts[0].parse::<i32>().unwrap();
    let end = parts[1].parse::<i32>().unwrap();
    return start..=end;
}

fn load_data() -> Vec<ElfPair> {
    let file = File::open("./data/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut elf_pairs = Vec::new();

    for line in lines {
        if let Ok(l) = line {
            let str_ranges = l.split(',').collect::<Vec<&str>>();

            elf_pairs.push(ElfPair{
                elf1: str_to_range(str_ranges[0].to_string()),
                elf2: str_to_range(str_ranges[1].to_string()),
            });
        }
    }

    return elf_pairs;
}