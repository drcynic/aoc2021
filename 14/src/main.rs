use std::collections::HashMap;
use std::fs;
use std::iter::*;

use itertools::Itertools;

fn main() {
    let filename = "input1.txt";
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
    for _ in 1..=10 {
        polymer = apply_instruction(&polymer, &instructions);
    }
    // println!("polymer {}: {:?}", i, polymer);

    let mut keys = polymer.chars().counts().into_iter().sorted_by_key(|(_, count)| *count);
    let least = keys.next().unwrap().1;
    let most = keys.last().unwrap().1;
    println!("least: {}", least);
    println!("most: {}", most);
    println!("part 1 - difference: {}", most - least);

    // part2 - do it more clever without string appending
    let mut pairs: HashMap<String, usize> = HashMap::new();
    start.chars().tuple_windows::<(_, _)>().for_each(|(fst, snd)| {
        let key = format!("{}{}", fst, snd);
        *pairs.entry(key).or_insert(0) += 1;
    });
    // println!("pairs: {:?}", pairs);

    for _ in 1..=40 {
        let mut update = HashMap::new();
        for (k, v) in pairs.iter() {
            let key = format!("{}{}", k.chars().nth(0).unwrap(), instructions.get(k.as_str()).unwrap());
            *update.entry(key).or_insert(0) += v;
            let key = format!("{}{}", instructions.get(k.as_str()).unwrap(), k.chars().nth(1).unwrap());
            *update.entry(key).or_insert(0) += v;
        }
        pairs = update;
    }

    // println!("pairs: {:?}", pairs);
    let mut counts = HashMap::new();
    for (k, v) in pairs.iter() {
        let k: &str = k.as_ref();
        *counts.entry(k.chars().nth(0).unwrap()).or_insert(0) += v;
    }
    *counts.entry(start.chars().last().unwrap()).or_insert(0) += 1;
    let mut keys = counts.into_iter().sorted_by_key(|(_, count)| *count);
    let least = keys.next().unwrap().1;
    let most = keys.last().unwrap().1;
    println!("least: {}", least);
    println!("most: {}", most);
    println!("part 2 - difference: {}", most - least);
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
