use std::fs;
use std::iter::*;

//use itertools::Itertools;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let width = input.lines().next().unwrap().len();
    let lines = input.lines().collect::<Vec<&str>>();
    let gamma_rate: u32 =
        get_bits(lines.as_slice(), width)
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (a, b))| if a > b { acc | 1 << (width - i - 1) } else { acc });
    let epsilon_rate = !gamma_rate & ((1 << width) - 1);
    println!("part1: {:?}", gamma_rate * epsilon_rate);

    // part2
    let oxy = get_rating(lines.clone(), width, (1, 0));
    let co2 = get_rating(lines, width, (0, 1));
    println!("part2: {:?}", oxy * co2);
}

fn get_rating(lines: Vec<&str>, width: usize, bits: (u32, u32)) -> u32 {
    let mut lines = lines.clone();
    let mut idx = 0;
    while lines.len() > 1 {
        let rate = get_bits(&lines, width);
        let common = if rate[idx].0 >= rate[idx].1 { bits.0 } else { bits.1 };
        lines = lines
            .clone()
            .into_iter()
            //.inspect(|line| println!("idx: {}, common: {}, line: {:?}", idx, common, line))
            .filter(|line| line.chars().nth(idx).unwrap() as u32 == common + 48)
            .collect();
        idx += 1;
    }
    to_number(lines[0])
}

fn get_bits(lines: &[&str], width: usize) -> Vec<(u32, u32)> {
    lines.iter().fold(vec![(0, 0); width], |mut acc, line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                acc[i].0 += 1;
            } else {
                acc[i].1 += 1;
            }
        });
        acc
    })
}

fn to_number(bits: &str) -> u32 {
    bits.chars().fold(0, |acc, c| (acc << 1) | (c as u32 - 48))
}
