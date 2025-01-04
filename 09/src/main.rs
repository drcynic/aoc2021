//use memoize::memoize;
//use std::collections::BTreeSet;
//use std::collections::HashMap;
use std::fs;
use std::iter::*;

use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let height_grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect())
        .collect();

    let low_points: Vec<(usize, usize)> = height_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, h)| {
                    let valid = [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().all(|d| {
                        let (dx, dy) = d;
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        nx < 0
                            || nx >= height_grid[y].len() as i32
                            || ny < 0
                            || ny >= height_grid.len() as i32
                            || height_grid[ny as usize][nx as usize] > *h
                    });

                    if valid {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();
    let part1 = low_points.iter().map(|(x, y)| 1 + height_grid[*y][*x]).sum::<i32>();
    println!("part1: {}", part1);

    let part2 = low_points
        .iter()
        .map(|(px, py)| collect_basin_count(&height_grid, px, py))
        .collect::<Vec<i32>>()
        .iter()
        .sorted()
        .rev()
        .take(3)
        .product::<i32>();
    println!("part2: {}", part2);
}

fn collect_basin_count(height_grid: &[Vec<i32>], px: &usize, py: &usize) -> i32 {
    let mut visited = vec![vec![false; height_grid[0].len()]; height_grid.len()];
    let mut queue = vec![(*px, *py)];
    let mut count = 0;
    while let Some((x, y)) = queue.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        count += 1;
        for d in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let (dx, dy) = d;
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0
                || nx >= height_grid[y].len() as i32
                || ny < 0
                || ny >= height_grid.len() as i32
                || height_grid[ny as usize][nx as usize] == 9
            {
                continue;
            }

            let entry = (nx as usize, ny as usize);
            queue.push(entry);
        }
    }
    count
}
