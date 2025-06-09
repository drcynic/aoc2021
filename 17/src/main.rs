use std::collections::HashSet;

fn main() {
    let y_min = -10;
    let y_max = -5;
    let x_min = 20;
    let x_max = 30;
    let mut max_y_pos = y_min;
    let mut distinct_velocities: HashSet<(i32, i32)> = HashSet::new();

    for start_vy in y_min..=-y_min {
        for start_vx in 0..=x_max {
            let mut local_y_max = y_min;
            let mut px = 0;
            let mut py = 0;
            let mut vx = start_vx;
            let mut vy = start_vy;
            loop {
                px += vx;
                py += vy;
                if px > x_max || py < y_min {
                    break;
                }
                if py > local_y_max {
                    local_y_max = py;
                }
                if px >= x_min && px <= x_max && py >= y_min && py <= y_max {
                    if local_y_max > max_y_pos {
                        max_y_pos = local_y_max;
                    }
                    distinct_velocities.insert((start_vx, start_vy));
                    break;
                }
                vx = std::cmp::max(vx - 1, 0);
                vy -= 1;
            }
        }
    }
    println!("Max y position: {}", max_y_pos);
    println!("Number of distinct velocities: {}", distinct_velocities.len());
}
