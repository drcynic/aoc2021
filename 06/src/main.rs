use memoize::memoize;
use std::fs;
use std::iter::*;

//use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let fishes: Vec<i64> = input.split(",").filter_map(|n| n.parse::<i64>().ok()).collect();

    let part1: usize = fishes.iter().map(|f| count_for_days(*f, 80) as usize).sum();
    println!("part1: {:?}", part1);

    let part2: usize = fishes.iter().map(|f| count_for_days(*f, 256) as usize).sum();
    println!("part2: {:?}", part2);
}

#[memoize]
fn count_for_days(fish: i64, days: i64) -> i64 {
    if fish >= days {
        return 1;
    }

    count_for_days(6, days - fish - 1) + count_for_days(8, days - fish - 1)
}
