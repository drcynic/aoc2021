use std::fs;
use std::iter::*;

use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let part1: i64 = input.lines().filter_map(is_corrupt).sum();
    println!("part1: {}", part1);

    let s: Vec<i64> = input
        .lines()
        .filter(|l| is_corrupt(l).is_none())
        .map(|l| {
            let mut stack = Vec::new();
            for c in l.chars() {
                match c {
                    '(' | '[' | '{' | '<' => {
                        stack.push(c);
                    }
                    ')' | ']' | '}' | '>' => {
                        stack.pop();
                    }
                    _ => unreachable!(),
                }
            }
            stack.iter().rev().fold(0, |acc, c| {
                let v = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
                5 * acc + v
            })
        })
        .sorted()
        .collect();
    let part2 = s[s.len() / 2];
    println!("part2: {}", part2);
}

fn is_corrupt(l: &str) -> Option<i64> {
    let mut stack = Vec::new();
    for c in l.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if let Some(v) = stack.pop() {
                    if v != '(' {
                        return Some(3);
                    }
                }
            }
            ']' => {
                if let Some(v) = stack.pop() {
                    if v != '[' {
                        return Some(57);
                    }
                }
            }
            '}' => {
                if let Some(v) = stack.pop() {
                    if v != '{' {
                        return Some(1197);
                    }
                }
            }
            '>' => {
                if let Some(v) = stack.pop() {
                    if v != '<' {
                        return Some(25137);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    None
}
