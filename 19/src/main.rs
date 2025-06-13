use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Pos { x, y, z }
    }

    pub fn add(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub rotation: usize,
    pub translation: Pos,
}

impl Transform {
    pub fn new(rotation: usize, translation: Pos) -> Self {
        Transform { rotation, translation }
    }

    pub fn apply(&self, point: &Pos) -> Pos {
        let rotated = apply_rotation(point, self.rotation);
        rotated.add(&self.translation)
    }

    pub fn apply_to_points(&self, points: &HashSet<Pos>) -> HashSet<Pos> {
        points.iter().map(|p| self.apply(p)).collect()
    }
}

#[derive(Debug, Default)]
struct Scanner {
    beacons: HashSet<Pos>,
}

fn main() {
    let filename = "input2.txt";
    let input_str = fs::read_to_string(filename).unwrap().trim().to_string();
    let scanner_str: Vec<&str> = input_str.split("\n\n").collect();
    let mut scanner: Vec<Scanner> = scanner_str
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
            ..Default::default()
        })
        .collect();

    let mut scanner0 = scanner.remove(0);
    let target_points = &mut scanner0.beacons;
    while scanner.len() > 0 {
        for j in 0..scanner.len() {
            let source_points = &scanner[j].beacons;
            if let Some(transform) = find_transform(source_points, target_points) {
                // println!("transform found, merging beacons");
                let transformed_points = transform.apply_to_points(&source_points);
                target_points.extend(transformed_points);
                scanner.remove(j);
                break;
            }
        }
    }
    println!("{} beacons in scanner 0", scanner0.beacons.len());
}

// All 24 possible 90-degree rotations
pub fn apply_rotation(point: &Pos, rotation: usize) -> Pos {
    let (x, y, z) = (point.x, point.y, point.z);

    match rotation {
        // Facing +X (6 rotations)
        0 => Pos::new(x, y, z),
        1 => Pos::new(x, -z, y),
        2 => Pos::new(x, -y, -z),
        3 => Pos::new(x, z, -y),

        // Facing -X (6 rotations)
        4 => Pos::new(-x, -y, z),
        5 => Pos::new(-x, -z, -y),
        6 => Pos::new(-x, y, -z),
        7 => Pos::new(-x, z, y),

        // Facing +Y (6 rotations)
        8 => Pos::new(-y, x, z),
        9 => Pos::new(z, x, y),
        10 => Pos::new(y, x, -z),
        11 => Pos::new(-z, x, -y),

        // Facing -Y (6 rotations)
        12 => Pos::new(y, -x, z),
        13 => Pos::new(z, -x, -y),
        14 => Pos::new(-y, -x, -z),
        15 => Pos::new(-z, -x, y),

        // Facing +Z (6 rotations)
        16 => Pos::new(-z, y, x),
        17 => Pos::new(-y, -z, x),
        18 => Pos::new(z, -y, x),
        19 => Pos::new(y, z, x),

        // Facing -Z (6 rotations)
        20 => Pos::new(z, y, -x),
        21 => Pos::new(-y, z, -x),
        22 => Pos::new(-z, -y, -x),
        23 => Pos::new(y, -z, -x),

        _ => panic!("Invalid rotation index: {}", rotation),
    }
}

pub fn find_transform(source_points: &HashSet<Pos>, target_points: &HashSet<Pos>) -> Option<Transform> {
    for rotation in 0..24 {
        let rotated_points: Vec<Pos> = source_points.iter().map(|p| apply_rotation(p, rotation)).collect();
        let mut translation_counts = HashMap::new();

        // For each pair of points, calculate the required translation
        for &rotated_point in rotated_points.iter() {
            for &target_point in target_points.iter() {
                let translation = target_point.sub(&rotated_point);
                *translation_counts.entry(translation).or_insert(0) += 1;
            }
        }

        if let Some((&best_translation, count)) = translation_counts.iter().max_by_key(|(_, count)| *count) {
            if *count >= 12 {
                return Some(Transform::new(rotation, best_translation));
            }
        }
    }

    None
}
