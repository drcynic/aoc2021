//use memoize::memoize;
use std::collections::HashMap;
use std::fs;
use std::iter::*;

//use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let crabs: Vec<i64> = input.split(",").filter_map(|n| n.parse::<i64>().ok()).collect();
    let count_by_crab: HashMap<i64, i64> = crabs.iter().fold(HashMap::new(), |mut acc, &crab| {
        *acc.entry(crab).or_insert(0) += 1;
        acc
    });

    let min = count_by_crab
        .keys()
        .map(|&ref_crab| {
            count_by_crab
                .iter()
                .map(|(crab, count)| (ref_crab - crab).abs() * count)
                .sum::<i64>()
        })
        .min()
        .unwrap();
    println!("part1: {:?}", min);

    let max_crab = *count_by_crab.keys().max().unwrap();
    let part2 = (0..=max_crab)
        .map(|ref_pos| {
            count_by_crab
                .iter()
                .map(|(crab, count)| {
                    let n = (ref_pos - crab).abs();
                    count * (n * n + n) / 2 // gaussche summenformel
                })
                .sum::<i64>()
        })
        .min()
        .unwrap();
    println!("part2: {:?}", part2);
}
