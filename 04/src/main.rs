use std::fs;
use std::iter::*;

//use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let (numbers_input, boards_input) = input.split_once("\n\n").unwrap();
    let numbers = numbers_input.split(',').filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
    let boards = boards_input
        .split("\n\n")
        .map(|bi| {
            let board: [[i32; 5]; 5] = bi.lines().enumerate().fold([[0; 5]; 5], |mut acc, (y, l)| {
                l.split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .enumerate()
                    .for_each(|(x, n)| acc[y][x] = n);
                acc
            });
            board
        })
        .collect::<Vec<[[i32; 5]; 5]>>();

    // part1 - find the first number that creates a bingo
    let mut end = 4;
    loop {
        if let Some(bingo_board) = boards.iter().find(|board| check_bingo(board, &numbers[..end])) {
            let part1 = eval_board(&numbers, end, bingo_board);
            println!("part1: {}", part1);
            break;
        }
        end += 1;
    }

    // part2 - find the first number that creates a bingo for all boards
    let mut end = 4;
    while !boards.iter().all(|board| check_bingo(board, &numbers[..end])) {
        end += 1;
    }
    boards
        .iter()
        .filter(|board| !check_bingo(board, &numbers[..end - 1]))
        .for_each(|bingo_board| {
            let part2 = eval_board(&numbers, end, bingo_board);
            println!("part2: {}", part2);
        });
}

fn eval_board(numbers: &[i32], end: usize, board: &[[i32; 5]; 5]) -> i32 {
    let missing_numbers: Vec<i32> = board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|n| !numbers[..end].contains(n))
        .copied()
        .collect();
    missing_numbers.iter().sum::<i32>() * numbers[end - 1]
}

fn check_bingo(board: &[[i32; 5]; 5], numbers: &[i32]) -> bool {
    for i in 0..5 {
        if board[i].iter().all(|&n| numbers.contains(&n)) {
            return true;
        }
        if board.iter().all(|r| numbers.contains(&r[i])) {
            return true;
        }
    }
    false
}
