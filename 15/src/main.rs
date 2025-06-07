use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pos: (i32, i32),
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let current_pos = Pos { pos: (0, 0) };

    let part1 = bfs(&grid, 1, current_pos);
    println!("part1: {}", part1);

    let part2 = bfs(&grid, 5, current_pos);
    println!("part2: {}", part2);
    // print_risks(&grid, 5);
}

fn bfs(grid: &[Vec<u8>], multiplier: usize, start_pos: Pos) -> i32 {
    let mut visited: HashMap<Pos, i32> = HashMap::new();
    let mut pq = PriorityQueue::new();
    pq.push(start_pos, Reverse(0));
    while let Some((current, cost)) = pq.pop() {
        if let Some(vc) = visited.get(&current) {
            if *vc <= cost.0 {
                continue;
            }
        }
        visited.insert(current, cost.0);

        let (x, y) = current.pos;
        if current.pos == ((grid[0].len() * multiplier) as i32 - 1, (grid.len() * multiplier) as i32 - 1) {
            return cost.0;
        }

        insert_or_update(grid, multiplier, &mut pq, (x - 1, y), cost);
        insert_or_update(grid, multiplier, &mut pq, (x + 1, y), cost);
        insert_or_update(grid, multiplier, &mut pq, (x, y - 1), cost);
        insert_or_update(grid, multiplier, &mut pq, (x, y + 1), cost);
    }

    0
}

fn insert_or_update(grid: &[Vec<u8>], multiplier: usize, pq: &mut PriorityQueue<Pos, Reverse<i32>>, pos: (i32, i32), cost: Reverse<i32>) {
    let (x, y) = pos;
    let grid_size_x = grid[0].len();
    let grid_size_y = grid.len();
    if x < 0 || x >= (grid_size_x * multiplier) as i32 || y < 0 || y >= (grid_size_y * multiplier) as i32 {
        return;
    }
    let x_grid = x % grid_size_x as i32;
    let y_grid = y % grid_size_y as i32;
    let x_risk_add = x / grid_size_x as i32;
    let y_risk_add = y / grid_size_y as i32;
    let mut risk = grid[y_grid as usize][x_grid as usize] as i32 + x_risk_add + y_risk_add;
    if risk > 9 {
        risk -= 9;
    }
    let cost = Reverse(cost.0 + risk);
    let pos = Pos { pos };
    if let Some(e) = pq.get(&pos) {
        if *e.1 < cost {
            pq.change_priority(&pos, cost);
        }
    } else {
        pq.push(pos, cost);
    }
}

#[allow(dead_code)]
fn print_risks(grid: &[Vec<u8>], multiplier: usize) {
    for y in 0..(grid.len() * multiplier) {
        for x in 0..(grid[0].len() * multiplier) {
            let x_grid = x % grid[0].len();
            let y_grid = y % grid.len();
            let x_risk_add = (x / grid[0].len()) as i32;
            let y_risk_add = (y / grid.len()) as i32;
            let mut risk = grid[y_grid as usize][x_grid as usize] as i32 + x_risk_add + y_risk_add;
            if risk > 9 {
                risk -= 9;
            }
            print!("{:>2}", risk);
        }
        println!();
    }
}
