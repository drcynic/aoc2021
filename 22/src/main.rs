use itertools::Itertools;
use std::{collections::HashSet, fs};

#[derive(Copy, Clone, Debug)]
struct Range {
    state: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

fn main() {
    let filename = "input1.txt";
    let input_str = fs::read_to_string(filename).unwrap().trim().to_string();
    let ranges = input_str
        .split("\n")
        .map(|line| {
            let (state, line) = line.split_once(" ").unwrap();
            let state = if state == "on" { true } else { false };
            let mut parts = line.split(",");
            let extract_range = |s: &str| -> (i64, i64) {
                s.split("=")
                    .nth(1)
                    .unwrap()
                    .split("..")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            };
            let x = extract_range(&parts.next().unwrap());
            let y = extract_range(&parts.next().unwrap());
            let z = extract_range(&parts.next().unwrap());
            Range { state, x, y, z }
        })
        .collect_vec();
    // println!("{:?}", ranges);

    part1(&ranges);
    part2(&ranges);
}

fn intersect(r1: &Range, r2: &Range) -> Option<Range> {
    let x_start = std::cmp::max(r1.x.0, r2.x.0);
    let x_end = std::cmp::min(r1.x.1, r2.x.1);

    let y_start = std::cmp::max(r1.y.0, r2.y.0);
    let y_end = std::cmp::min(r1.y.1, r2.y.1);

    let z_start = std::cmp::max(r1.z.0, r2.z.0);
    let z_end = std::cmp::min(r1.z.1, r2.z.1);

    Some(Range {
        x: (x_start, x_end),
        y: (y_start, y_end),
        z: (z_start, z_end),
        state: !r1.state,
    })
    .filter(|r| r.x.0 <= r.x.1 && r.y.0 <= r.y.1 && r.z.0 <= r.z.1)
}

fn part2(ranges: &[Range]) {
    let mut processed_ranges = vec![ranges[0].clone()];
    for i in 1..ranges.len() {
        let mut new_ranges = Vec::new();
        for proc_range in &mut processed_ranges {
            if let Some(new_range) = intersect(proc_range, &ranges[i]) {
                // store the intersection range with opposite on/off of split_range to compensate later on
                new_ranges.push(new_range);
            }
        }
        processed_ranges.extend(new_ranges);
        if ranges[i].state {
            processed_ranges.push(ranges[i].clone()); // if 'on' then add the block
        }
    }

    let num_range = |r: &Range| (r.x.1 - r.x.0 + 1) * (r.y.1 - r.y.0 + 1) * (r.z.1 - r.z.0 + 1) * if r.state { 1 } else { -1 };
    let cpus_on = processed_ranges.iter().map(num_range).sum::<i64>();
    println!("part2: {}", cpus_on);
}

fn part1(ranges: &[Range]) {
    let mut cpus: HashSet<(i64, i64, i64)> = HashSet::new();
    for range in ranges {
        let mut range = range.clone();
        range.x.0 = range.x.0.max(-50);
        range.x.1 = range.x.1.min(50);
        range.y.0 = range.y.0.max(-50);
        range.y.1 = range.y.1.min(50);
        range.z.0 = range.z.0.max(-50);
        range.z.1 = range.z.1.min(50);
        for z in range.z.0..=range.z.1 {
            for y in range.y.0..=range.y.1 {
                for x in range.x.0..=range.x.1 {
                    if range.state {
                        cpus.insert((x, y, z));
                    } else {
                        cpus.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    println!("part1: {}", cpus.len());
}
