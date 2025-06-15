use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let filename = "input1.txt";
    let input_str = fs::read_to_string(filename).unwrap().trim().to_string();
    let (algo_str, map_str) = input_str.split_once("\n\n").unwrap();
    let algo: Vec<bool> = algo_str.chars().map(|c| c == '#').collect();
    let mut map: HashSet<(i32, i32)> = map_str.lines().enumerate().fold(HashSet::new(), |mut acc, (line_idx, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                acc.insert((x as i32, line_idx as i32));
            }
        });
        acc
    });

    for i in 0..50 {
        map = apply_filter(&algo, &map, i);
        if i == 1 {
            println!("lit 2: {}", map.len());
        }
    }
    println!("lit 50: {}", map.len());
    // print_map(&map);
}

fn apply_filter(algo: &[bool], map: &HashSet<(i32, i32)>, run: i32) -> HashSet<(i32, i32)> {
    let (min_x, min_y, max_x, max_y) = map_extends(&map);
    let outside = run % 2 == 1 && algo[0];
    let mut new_map: HashSet<(i32, i32)> = HashSet::new();
    for y in min_y - 1..=max_y + 1 {
        for x in min_x - 1..=max_x + 1 {
            let mut idx = 0;
            let mut i = 0usize;
            for yf in (y - 1)..=y + 1 {
                for xf in (x - 1)..=x + 1 {
                    if map.contains(&(xf, yf)) || ((xf < min_x || xf > max_x || yf < min_y || yf > max_y) && outside) {
                        idx |= 1 << (8 - i);
                    }
                    i += 1;
                }
            }
            if algo[idx] {
                new_map.insert((x, y));
            }
        }
    }
    new_map
}

fn map_extends(map: &HashSet<(i32, i32)>) -> (i32, i32, i32, i32) {
    let (min_x, max_x) = map.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    (*min_x, *min_y, *max_x, *max_y)
}

#[allow(dead_code)]
fn print_map(map: &HashSet<(i32, i32)>) {
    let (min_x, min_y, max_x, max_y) = map_extends(map);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if map.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
}
