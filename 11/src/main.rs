use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::*;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let mut octopuses: HashMap<(i32, i32), i32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, c)| Some(((x as i32, y as i32), c.to_digit(10)? as i32)))
        })
        .collect();

    let mut sum = 0;
    let mut octopuses_p1 = octopuses.clone();
    for _ in 1..=100 {
        sum += step(&mut octopuses_p1);
    }
    println!("part1: {:?}", sum);

    let mut num_steps = 1;
    while step(&mut octopuses) < 100 {
        num_steps += 1;
    }
    println!("part2: {:?}", num_steps);
}

fn step(octopuses: &mut HashMap<(i32, i32), i32>) -> i32 {
    //print_octupuses(octopuses);
    octopuses.values_mut().for_each(|level| {
        *level += 1;
    });

    let mut flashed: HashSet<(i32, i32)> = HashSet::new();
    let mut flashing_octopuses: Vec<(i32, i32)> = get_flashing_ones(octopuses, &flashed);
    while !flashing_octopuses.is_empty() {
        for &(x, y) in flashing_octopuses.iter() {
            flashed.insert((x, y));
            for d in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter() {
                let pos = (x + d.0, y + d.1);
                if flashed.contains(&pos) {
                    continue;
                }
                if let Some(level) = octopuses.get_mut(&pos) {
                    *level += 1;
                }
            }
        }
        flashing_octopuses = get_flashing_ones(octopuses, &flashed);
    }

    flashed.iter().for_each(|pos| {
        if let Some(level) = octopuses.get_mut(pos) {
            *level = 0;
        }
    });

    flashed.len() as i32
}

fn get_flashing_ones(octopuses: &HashMap<(i32, i32), i32>, flashed: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    octopuses
        .iter()
        .filter_map(|(pos, level)| if *level > 9 && !flashed.contains(pos) { Some(*pos) } else { None })
        .collect::<Vec<(i32, i32)>>()
}

fn print_octupuses(octopuses: &HashMap<(i32, i32), i32>) {
    for y in 0..10 {
        for x in 0..10 {
            if let Some(level) = octopuses.get(&(x, y)) {
                print!("{}", level);
            }
        }
        println!();
    }
}
