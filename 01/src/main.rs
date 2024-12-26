use std::fs;
use std::iter::zip;

use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let part1: i32 = input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(c, n)| if n > c { 1 } else { 0 })
        .sum();
    println!("{:?}", part1);
    let part2: i32 = input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(c, n, nn)| c + n + nn)
        .tuple_windows()
        .map(|(c, n)| if n > c { 1 } else { 0 })
        .sum();
    println!("{:?}", part2);
}
