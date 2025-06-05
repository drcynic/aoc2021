use std::collections::HashMap;
use std::fs;
use std::iter::*;

use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let (start, instr) = input.split_once("\n\n").unwrap();
    println!("start: {:?}", start);
    let instructions: HashMap<&str, &str> = instr
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once(" -> ").unwrap())
        .collect();
    // println!("instructions: {:?}", instructions);

    let mut polymer = start.to_string();
    for _ in 0..10 {
        polymer = apply_instruction(&polymer, &instructions);
    }
    // println!("polymer: {:?}", polymer);
    let mut keys = polymer.chars().counts().into_iter().sorted_by_key(|(_, count)| *count);
    let least = keys.next().unwrap().1;
    let most = keys.last().unwrap().1;
    println!("least: {}", least);
    println!("most: {}", most);
    println!("difference: {}", most - least);
}

fn apply_instruction(current: &str, instr: &HashMap<&str, &str>) -> String {
    let mut new_variant = "".to_string();

    current.chars().tuple_windows::<(_, _)>().for_each(|(fst, snd)| {
        new_variant.push(fst);
        let key = format!("{}{}", fst, snd);
        if let Some(val) = instr.get(key.as_str()) {
            new_variant.push(val.chars().nth(0).unwrap());
        }
    });
    new_variant.push(current.chars().last().unwrap());
    new_variant
}
