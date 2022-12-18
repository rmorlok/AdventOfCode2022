extern crate core;

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Clone, Copy)]
struct Action {
    num: u32,
    from: usize,
    to: usize,
}

struct Stack {
    crates: Vec<char>
}

impl Stack {
    fn new() -> Stack {
        return Stack {
            crates: Vec::new()
        };
    }

    fn add_crate(&mut self, c: char) {
        self.crates.push(c);
    }

    fn add_crates(&mut self, crates: &Vec<char>) {
        for i in (0..crates.len()).rev() {
            self.add_crate(*crates.get(i).unwrap());
        }
    }

    fn remove_crates(&mut self, num: u32) -> Vec<char> {
        let mut crates = Vec::new();

        for _ in 0..num {
            crates.push(self.crates.pop().unwrap());
        }

        return crates;
    }

    fn remove_crate(&mut self) -> char {
        return self.crates.pop().unwrap();
    }

    fn get_crates(&self) -> &[char] {
        return self.crates.as_slice();
    }

    fn get_top(&self) -> char {
        return match self.crates.last() {
            Some(c) => *c,
            None => ' ',
        };
    }
}

struct Ship {
    stacks: Vec<Stack>
}

impl Ship {
    fn new(num_stacks: u32) -> Ship {
        return Ship{
            stacks: (0..num_stacks).map(|_| Stack::new()).collect()
        }
    }

    fn add_crate_to_stack(&mut self, stack: u32, c: char) {
        let s = self.stacks.get_mut(stack as usize).unwrap();
        s.add_crate(c);
    }

    fn get_stack(&self, stack: u32) -> &Stack {
        return self.stacks.get(stack as usize).unwrap();
    }

    fn get_stack_mut(&mut self, stack: u32) -> &mut Stack {
        return self.stacks.get_mut(stack as usize).unwrap();
    }

    fn move_crates(&mut self, from_pos: u32, to_pos: u32, num_crates: u32) {
        let (from, to) = (from_pos - 1, to_pos - 1);
        if from == to {
            return;
        } else if from < to {
            let (f, t) = self.stacks.split_at_mut(from as usize + 1);
            let from_stack = f.get_mut(from as usize).unwrap();
            let to_stack = t.get_mut((to - from - 1) as usize).unwrap();
            to_stack.add_crates(&from_stack.remove_crates(num_crates));
        } else {
            let (t, f) = self.stacks.split_at_mut(to as usize + 1);
            let from_stack = f.get_mut((from - to - 1) as usize).unwrap();
            let to_stack = t.get_mut(to as usize).unwrap();
            to_stack.add_crates(&from_stack.remove_crates(num_crates));
        }
    }

    fn execute(&mut self, action: &Action, at_once: bool) {
        if !at_once {
            for i in 0..action.num {
                self.move_crates(action.from as u32, action.to as u32, 1);
            }
        } else {
            self.move_crates(action.from as u32, action.to as u32, action.num);
        }

    }

    fn top_of_each_stack(&self) -> String {
        return String::from_iter(self.stacks.iter().map(|s| s.get_top()));
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, s) in self.stacks.iter().enumerate() {
            write!(f, "Stack {}:", i + 1);
            for c in s.get_crates().iter() {
                write!(f, " [{}]", c);
            }
            write!(f, "\n");
        }

        write!(f, "")
    }
}

fn part_1() {
    println!("====================== Part 1 ======================");
    let (mut ship, actions) = load_data();
    println!("Start:");
    println!("{}", ship);
    println!("Top of each stacK: {}", ship.top_of_each_stack());

    for action in actions {
        ship.execute(&action, false);
    }

    println!("End:");
    println!("{}", ship);
    println!();
    println!("Top of each stacK: {}", ship.top_of_each_stack());
}

fn part_2() {
    println!("====================== Part 2 ======================");
    let (mut ship, actions) = load_data();
    println!("Start:");
    println!("{}", ship);
    println!("Top of each stacK: {}", ship.top_of_each_stack());

    for action in actions {
        ship.execute(&action, true);
    }

    println!("End:");
    println!("{}", ship);
    println!();
    println!("Top of each stacK: {}", ship.top_of_each_stack());
}

fn main() {
    part_1();
    part_2();
}

fn extract_ship(ship_lines: &[String]) -> Ship {
    let labels = ship_lines.last().unwrap();
    let labels_re = Regex::new(r"(\d+)").unwrap();
    let top_stack_cap = labels_re.captures_iter(labels).last().unwrap();
    let top_stack_num = ((&top_stack_cap[0])).parse::<u32>().unwrap();
    // let non_label_lines = &ship_lines[..ship_lines.len()-1];

    let mut ship = Ship::new(top_stack_num);
    for i in (0..ship_lines.len() - 1).rev() {
        for s  in 0..(top_stack_num as usize) {
            let line = &ship_lines[i];
            let mut c_str = line.as_str()[(4*s)..=(4*(s+1)-2)].chars();
            if  c_str.next().unwrap() == '[' {
                ship.add_crate_to_stack(s as u32, c_str.next().unwrap());
            }
        }
    }

    return ship;
}

fn extract_actions(action_lines: &[String]) -> Vec<Action> {
    let action_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    return action_lines.to_vec().iter().map(|s| {
        let c = action_re.captures(s).unwrap();

        return Action{
            num: c[1].parse::<u32>().unwrap(),
            from: c[2].parse::<usize>().unwrap(),
            to: c[3].parse::<usize>().unwrap(),
        };
    }).collect();
}

fn load_data() -> (Ship, Vec<Action>) {
    let file = File::open("./data/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let parts = lines.split(|s| s.is_empty()).collect::<Vec<&[String]>>();

    return (extract_ship(parts[0]), extract_actions(parts[1]))
}