use itertools::{self, Itertools};
use std::fs;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Pos>,
}

fn print_distances(scanner: &Scanner) {
    scanner.beacons.iter().enumerate().for_each(|(i, p)| println!("{}: {:?}", i, &p));
    for i in 0..scanner.beacons.len() {
        for j in (i + 1)..scanner.beacons.len() {
            let dist_x = scanner.beacons[i].x - scanner.beacons[j].x;
            let dist_y = scanner.beacons[i].y - scanner.beacons[j].y;
            let dist_z = scanner.beacons[i].z - scanner.beacons[j].z;
            println!("Distance between beacon {} and beacon {}: {}/{}/{}", i, j, dist_x, dist_y, dist_z);
        }
    }
}

fn main() {
    let filename = "input1.txt";
    let input_str = fs::read_to_string(filename).unwrap().trim().to_string();
    let scanner_str: Vec<&str> = input_str.split("\n\n").collect();
    println!("scanner str: {:?}", &scanner_str);
    let all_scanner: Vec<Scanner> = scanner_str
        .into_iter()
        .map(|s| Scanner {
            beacons: s
                .split("\n")
                .into_iter()
                .skip(1)
                .map(|l| {
                    let a: Vec<&str> = l.split(",").collect();
                    Pos {
                        x: a[0].parse().unwrap(),
                        y: a[1].parse().unwrap(),
                        z: a[2].parse().unwrap(),
                    }
                })
                .collect(),
        })
        .collect();

    println!("all_scanner: {:?}", &all_scanner);

    all_scanner.into_iter().for_each(|scanner| {
        print_distances(&scanner);
    });
}
