use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Player {
    pos: u64,
    points: u64,
}

impl Player {
    fn new(pos: u64) -> Self {
        Player { pos: pos - 1, points: 0 }
    }

    fn mov(&mut self, step: u64) -> &Self {
        self.pos = (self.pos + step) % 10;
        self.add();
        self
    }

    fn add(&mut self) {
        self.points += self.pos + 1;
    }
}

fn main() {
    let p1 = Player::new(4);
    let p2 = Player::new(8);
    part1(&p1, &p2);
    part2(&p1, &p2);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Game {
    players: [Player; 2],
}

fn part2(p1: &Player, p2: &Player) {
    let dice_combinations = (1..=3)
        .flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c)))
        .collect::<Vec<u64>>();
    // println!("dice combinations: {:?}", dice_combinations);
    let mut universe_count_by_game = HashMap::from([(
        Game {
            players: [p1.clone(), p2.clone()],
        },
        1u64,
    )]);
    let mut player_wins = [0u64, 0u64];
    let mut pidx = 0;
    while !universe_count_by_game.is_empty() {
        let mut new_uni_by_game = HashMap::new();
        for steps in &dice_combinations {
            for (game, universe_count) in &universe_count_by_game {
                let mut players = game.players;
                if players[pidx].mov(*steps).points >= 21 {
                    player_wins[pidx] += universe_count;
                } else {
                    *new_uni_by_game.entry(Game { players }).or_insert(0) += universe_count;
                }
            }
        }
        universe_count_by_game = new_uni_by_game;
        pidx = (pidx + 1) % 2;
    }
    // println!("player wins: {:?}", &player_wins);
    println!("part2: {}", std::cmp::max(player_wins[0], player_wins[1]));
}

fn part1(p1: &Player, p2: &Player) {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();
    let mut dice = 0;
    let mut dice_rolls = 0;
    let min_points = 1000;
    while p1.points < min_points && p2.points < min_points {
        let dice_mov = roll_dice(&mut dice);
        p1.mov(dice_mov);
        dice_rolls += 3;
        if p1.points >= min_points {
            break;
        }
        let dice_mov = roll_dice(&mut dice);
        p2.mov(dice_mov);
        dice_rolls += 3;
    }
    // println!("p1 points: {}", p1.points);
    // println!("p2 points: {}", p2.points);
    // println!("dice rolls: {}", dice_rolls);
    println!("part1: {}", p1.points.min(p2.points) * dice_rolls);
}

fn roll_dice(dice: &mut u64) -> u64 {
    let mut sum = 0;
    for _ in 0..3 {
        *dice += 1;
        if *dice == 101 {
            *dice = 1;
        }
        sum += *dice;
    }
    sum
}
