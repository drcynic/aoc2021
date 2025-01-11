use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::*;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let caves: HashMap<&str, HashSet<&str>> = input.lines().fold(HashMap::new(), |mut acc, line| {
        let (start, end) = line.split_once("-").unwrap();
        acc.entry(start).or_default().insert(end);
        acc.entry(end).or_default().insert(start);
        acc
    });
    let part1 = dfs("start", &caves, &mut HashSet::new());
    println!("Part 1: {}", part1);
    let part2 = dfs2("start", &caves, &mut HashSet::new(), "");
    println!("Part 2: {}", part2);
}

fn is_small(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

fn dfs<'a>(current: &'a str, caves: &'a HashMap<&'a str, HashSet<&'a str>>, visited: &mut HashSet<&'a str>) -> i32 {
    if current == "end" {
        return 1;
    }
    if visited.contains(current) {
        return 0;
    }
    if is_small(current) {
        visited.insert(current);
    }

    let mut sum = 0;
    for cave in caves.get(current).unwrap() {
        sum += dfs(cave, caves, visited);
    }
    visited.remove(current);
    sum
}

fn dfs2<'a>(
    current: &'a str,
    caves: &'a HashMap<&'a str, HashSet<&'a str>>,
    visited: &mut HashSet<&'a str>,
    visited_twice: &'a str,
) -> i32 {
    if current == "end" {
        return 1;
    }

    let mut select_twice = false;
    if visited.contains(current) {
        if !visited_twice.is_empty() || current == "start" {
            return 0;
        }
        select_twice = true;
    } else if is_small(current) {
        visited.insert(current);
    }

    let mut sum = 0;
    let visited_twice = if select_twice { current } else { visited_twice };
    for cave in caves.get(current).unwrap() {
        sum += dfs2(cave, caves, visited, visited_twice);
    }

    if !select_twice {
        visited.remove(current);
    }

    sum
}
