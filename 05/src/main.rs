use std::collections::HashMap;
use std::fs;
use std::iter::*;

//use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let segs: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .map(|l| {
            let (s, e) = l.split_once(" -> ").unwrap();
            let (sx, sy) = s.split_once(",").unwrap();
            let (ex, ey) = e.split_once(",").unwrap();
            (
                (sx.parse::<i32>().unwrap(), sy.parse::<i32>().unwrap()),
                (ex.parse::<i32>().unwrap(), ey.parse::<i32>().unwrap()),
            )
        })
        .collect();

    let num = overlay_lines(&segs, true);
    println!("part1: {}", num);

    let num = overlay_lines(&segs, false);
    println!("part2: {}", num);
}

fn overlay_lines(segs: &[((i32, i32), (i32, i32))], skip_diagonal: bool) -> usize {
    let mut occupied_positions: HashMap<(i32, i32), i32> = HashMap::new();
    for ((sx, sy), (ex, ey)) in segs {
        let dx = ex - sx;
        let dy = ey - sy;
        if skip_diagonal && dx.abs() > 0 && dy.abs() > 0 {
            continue;
        }
        let steps = dx.abs().max(dy.abs());
        for i in 0..=steps {
            let x = sx + i * dx / steps;
            let y = sy + i * dy / steps;
            *occupied_positions.entry((x, y)).or_insert(0) += 1;
        }
    }
    occupied_positions.values().filter(|&&v| v > 1).count()
}
