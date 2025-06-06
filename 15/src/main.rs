use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pos: (i32, i32),
}

fn main() {
    let filename = "input1.txt";
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let current_pos = Pos { pos: (0, 0) };

    let part1 = bfs(&grid, current_pos);
    println!("part1: {}", part1);
}

fn bfs(grid: &[Vec<u8>], start_pos: Pos) -> i32 {
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
        if current.pos == (grid[0].len() as i32 - 1, grid.len() as i32 - 1) {
            return cost.0;
        }

        insert_or_update(grid, &mut pq, (x - 1, y), cost);
        insert_or_update(grid, &mut pq, (x + 1, y), cost);
        insert_or_update(grid, &mut pq, (x, y - 1), cost);
        insert_or_update(grid, &mut pq, (x, y + 1), cost);
    }

    0
}

fn insert_or_update(grid: &[Vec<u8>], pq: &mut PriorityQueue<Pos, Reverse<i32>>, pos: (i32, i32), cost: Reverse<i32>) {
    let (x, y) = pos;
    if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
        return;
    }

    let cost = Reverse(cost.0 + grid[y as usize][x as usize] as i32);
    let pos = Pos { pos };
    if let Some(e) = pq.get(&pos) {
        if *e.1 < cost {
            pq.change_priority(&pos, cost);
        }
    } else {
        pq.push(pos, cost);
    }
}
