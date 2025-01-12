use std::collections::HashSet;
use std::fs;
use std::iter::*;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let (dots_input, fold_input) = input.split_once("\n\n").unwrap();
    let mut dots: HashSet<(i32, i32)> = dots_input.lines().fold(HashSet::new(), |mut acc, line| {
        let (x, y) = line.split_once(",").unwrap();
        acc.insert((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
        acc
    });
    let mut max_x = *dots.iter().map(|(x, _)| x).max().unwrap();
    let mut max_y = *dots.iter().map(|(_, y)| y).max().unwrap();
    let folds: Vec<(bool, i32)> = fold_input
        .lines()
        .map(|line| {
            let (l, r) = line[11..].split_once("=").unwrap();
            (l == "x", r.parse::<i32>().unwrap())
        })
        .collect();
    folds.into_iter().for_each(|(is_x, fold_idx)| {
        let new_dots: HashSet<(i32, i32)> = dots.iter().fold(HashSet::new(), |mut acc, (x, y)| {
            if is_x {
                if *x > fold_idx {
                    let new_x = 2 * fold_idx - *x;
                    if new_x >= 0 {
                        acc.insert((new_x, *y));
                    }
                } else {
                    acc.insert((*x, *y));
                }
            } else if *y > fold_idx {
                let new_y = 2 * fold_idx - *y;
                if new_y >= 0 {
                    acc.insert((*x, new_y));
                }
            } else {
                acc.insert((*x, *y));
            }
            acc
        });
        dots = new_dots;
        max_x = *dots.iter().map(|(x, _)| x).max().unwrap();
        max_y = *dots.iter().map(|(_, y)| y).max().unwrap();
        println!("{}", dots.len()); // first output is part1
    });
    println!("part2: ");
    print_dots(&dots, max_x, max_y);
}

fn print_dots(dots: &HashSet<(i32, i32)>, max_x: i32, max_y: i32) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
