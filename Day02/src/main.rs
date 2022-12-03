extern crate core;

use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Winner {
    Player1,
    Player2,
    Tie,
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
struct Match {
    player1: Move,
    player2: Move,
}

fn main() {
    let part_1_final_score = load_data(&part_1)
        .iter()
        .map(|&m| match_score(m))
        .fold(0, |sum, ms| sum + ms);
    let part_2_final_score = load_data(&part_2)
        .iter()
        .map(|&m| match_score(m))
        .fold(0, |sum, ms| sum + ms);
    println!("Part 1 final score: {}", part_1_final_score);
    println!("Part 2 final score: {}", part_2_final_score);
}

fn match_score(m: Match) -> i32 {
    return move_score(m.player2) + match winner(m) {
        Winner::Player1 => 0,
        Winner::Player2 => 6,
        Winner::Tie => 3,
    }
}

fn move_score(m: Move) -> i32 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn winner(m: Match) -> Winner {
    return match (m.player1, m.player2) {
        (Move::Rock, Move::Rock) => Winner::Tie,
        (Move::Rock, Move::Paper) => Winner::Player2,
        (Move::Rock, Move::Scissors) => Winner::Player1,

        (Move::Paper, Move::Rock) => Winner::Player1,
        (Move::Paper, Move::Paper) => Winner::Tie,
        (Move::Paper, Move::Scissors) => Winner::Player2,

        (Move::Scissors, Move::Rock) => Winner::Player2,
        (Move::Scissors, Move::Paper) => Winner::Player1,
        (Move::Scissors, Move::Scissors) => Winner::Tie,
    }
}

fn load_data(interpreter: &dyn Fn(usize, char, char) -> Match) -> Vec<Match> {
    let file = File::open("./data/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut matches = Vec::new();

    for (i, line) in lines.enumerate() {
        if let Ok(m) = line {
            let v: Vec<&str> = m.split(' ').collect();
            matches.push(interpreter(
                i,
                v[0].chars().next().unwrap(),
                v[1].chars().next().unwrap())
            );
        }
    }

    return matches;
}

fn part_1(i: usize, c1: char, c2: char) -> Match {
    return Match{
        player1: match c1 {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            x => panic!("Error on line {}: Player 1 value '{}'", i, x),
        },
        player2: match c2 {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            x => panic!("Error on line {}: Player 2 value '{}'", i, x),
        }
    }
}

fn part_2(i: usize, c1: char, c2: char) -> Match {
    return Match{
        player1: match c1 {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            x => panic!("Error on line {}: Player 1 value '{}'", i, x),
        },
        player2: match (c1, c2) {
            ('A', 'X') => Move::Scissors,
            ('A', 'Y') => Move::Rock,
            ('A', 'Z') => Move::Paper,

            ('B', 'X') => Move::Rock,
            ('B', 'Y') => Move::Paper,
            ('B', 'Z') => Move::Scissors,

            ('C', 'X') => Move::Paper,
            ('C', 'Y') => Move::Scissors,
            ('C', 'Z') => Move::Rock,

            (_, x) => panic!("Error on line {}: Player 2 value '{}'", i, x),
        }
    }
}