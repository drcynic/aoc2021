//use memoize::memoize;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs;
use std::iter::*;

//use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let signals: Vec<(Vec<BTreeSet<char>>, [&str; 4])> = input
        .lines()
        .map(|l| {
            let (left_str, right_str) = l.split_once(" | ").unwrap();
            (
                left_str
                    .split_whitespace()
                    .map(|e| BTreeSet::from_iter(e.chars()))
                    .collect::<Vec<BTreeSet<char>>>(),
                right_str.split_whitespace().collect::<Vec<&str>>().try_into().unwrap(),
            )
        })
        .collect();
    //println!("{:?}", signals);

    let part1: i32 = signals
        .iter()
        .map(|(_, output_values)| {
            output_values
                .iter()
                .map(|v| match v.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum();
    println!("Part 1: {}", part1);

    let part2: i32 = signals
        .iter()
        .map(|(segments, digits)| {
            let result = unwire(segments);
            digits
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    let digit_set: BTreeSet<char> = BTreeSet::from_iter(d.chars());
                    let pos = result.iter().position(|s| s == &digit_set).unwrap();
                    let dec = 10u32.pow(3 - i as u32) as i32;
                    dec * pos as i32
                })
                .sum::<i32>()
        })
        .sum();
    println!("Part 2: {}", part2);
}

fn find_and_assign(signals: &mut Vec<BTreeSet<char>>, result: &mut [BTreeSet<char>; 10], digit: usize, len: usize) {
    let pos = signals.iter().position(|s| s.len() == len).unwrap();
    result[digit] = signals[pos].clone();
    signals.remove(pos);
}

fn find_intersection_and_assign(
    signals: &mut Vec<BTreeSet<char>>,
    result: &mut [BTreeSet<char>; 10],
    digit: usize,
    ref_number: usize,
    len: usize,
) {
    let pos = signals
        .iter()
        .position(|s| s.len() == len && s.intersection(&result[ref_number]).count() == result[ref_number].len())
        .unwrap();
    result[digit] = signals[pos].clone();
    signals.remove(pos);
}

// 1, 4, 7, 8 -> clear,
// 3 (with 5 segs) must contain 7, no other 5-seg number does
// 9 (with 6 segs) must contain 4, no other 6-seg number does
// 0 (with 6 segs) must contain 1 or 7, no other remaining 6-seg number does (6),
// 6 therefore clear as only 6-seg left
// 2 must contain diff of 8 and 9 (left lower seg) the other 5-seg (5) doesn't use this
// 5 the remaining 5-seg
fn unwire(signals: &[BTreeSet<char>]) -> [BTreeSet<char>; 10] {
    let mut signals: Vec<BTreeSet<char>> = signals.to_owned();
    let mut result: [BTreeSet<char>; 10] = vec![BTreeSet::new(); 10].try_into().unwrap();

    find_and_assign(&mut signals, &mut result, 1, 2); // 1
    find_and_assign(&mut signals, &mut result, 4, 4); // 4
    find_and_assign(&mut signals, &mut result, 7, 3); // 7
    find_and_assign(&mut signals, &mut result, 8, 7); // 8

    find_intersection_and_assign(&mut signals, &mut result, 3, 7, 5); // 3
    find_intersection_and_assign(&mut signals, &mut result, 9, 4, 6); // 9
    find_intersection_and_assign(&mut signals, &mut result, 0, 1, 6); // 0

    find_and_assign(&mut signals, &mut result, 6, 6); // 6

    // 2
    let diff_8_and_9 = result[8].difference(&result[9]).next().unwrap();
    let pos = signals.iter().position(|s| s.len() == 5 && s.contains(diff_8_and_9)).unwrap();
    result[2] = signals[pos].clone();
    signals.remove(pos);

    result[5] = signals[0].clone(); // 5

    result
}
