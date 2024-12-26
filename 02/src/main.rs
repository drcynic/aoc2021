use std::fs;
use std::iter::zip;

use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let pos = input
        .lines()
        .map(|l| {
            let (cmd, units) = l.split_once(" ").unwrap();
            let units = units.parse::<i32>().unwrap();
            match cmd {
                "forward" => (units, 0),
                "down" => (0, units),
                "up" => (0, -units),
                _ => unreachable!(),
            }
        })
        .fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy));
    println!("part1: {:?}", pos.0 * pos.1);

    let pos = input.lines().fold((0, 0, 0), |(x, y, aim), l| {
        let (cmd, units) = l.split_once(" ").unwrap();
        let units = units.parse::<i32>().unwrap();
        match cmd {
            "forward" => (x + units, y + aim * units, aim),
            "down" => (x, y, aim + units),
            "up" => (x, y, aim - units),
            _ => unreachable!(),
        }
    });
    println!("part2: {:?}", pos.0 * pos.1);
}
