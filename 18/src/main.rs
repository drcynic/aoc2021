use std::fs;

#[derive(Clone, Debug)]
enum Element {
    Open,
    Number(i32),
    Close,
}

fn parse(s: &str) -> Vec<Element> {
    s.chars()
        .filter(|c| *c != ',' && *c != ' ')
        .map(|c| match c {
            '[' => Element::Open,
            ']' => Element::Close,
            _ => Element::Number(c.to_digit(10).unwrap() as i32),
        })
        .collect::<Vec<Element>>()
}

fn to_string(e: &Vec<Element>) -> String {
    let mut result = String::new();
    let mut need_comma = false;

    for element in e {
        match element {
            Element::Open => {
                if need_comma {
                    result.push(',');
                }
                result.push('[');
                need_comma = false;
            }
            Element::Close => {
                result.push(']');
                need_comma = true;
            }
            Element::Number(n) => {
                if need_comma {
                    result.push(',');
                }
                result.push_str(&n.to_string());
                need_comma = true;
            }
        }
    }

    result
}

fn add(left: &[Element], right: &[Element]) -> Vec<Element> {
    let mut added = Vec::new();
    added.push(Element::Open);
    added.extend_from_slice(left);
    added.extend_from_slice(right);
    added.push(Element::Close);
    added
}

fn explode(fish: &mut Vec<Element>) -> bool {
    let mut left_num_idx: Option<usize> = None;
    let mut left_val = 0;
    let mut num_open = 0;
    for (i, e) in fish.iter().enumerate() {
        match *e {
            Element::Open => num_open += 1,
            Element::Close => num_open -= 1,
            Element::Number(n) => {
                left_num_idx = Some(i);
                left_val = n;
            }
        }

        if num_open >= 5 {
            if let Element::Number(lv) = fish[i + 1] {
                if let Element::Number(rv) = fish[i + 2] {
                    if let Some(prev_idx) = left_num_idx {
                        fish[prev_idx] = Element::Number(lv + left_val);
                    }
                    fish.drain((i + 1)..(i + 4));

                    for j in i..fish.len() {
                        if let Element::Number(n) = fish[j] {
                            fish[j] = Element::Number(n + rv);
                            break;
                        }
                    }
                    fish[i] = Element::Number(0);
                    return true;
                }
            }
        }
    }
    false
}

fn split(fish: &mut Vec<Element>) -> bool {
    for (i, e) in fish.iter().enumerate() {
        match *e {
            Element::Number(n) => {
                if n >= 10 {
                    let l = n / 2;
                    let r = if n & 1 == 0 { n / 2 } else { n / 2 + 1 };
                    fish.insert(i + 1, Element::Close);
                    fish.insert(i + 1, Element::Number(r));
                    fish.insert(i + 1, Element::Number(l));
                    fish[i] = Element::Open;
                    return true;
                }
            }
            _ => (),
        }
    }

    false
}

fn reduce(fish: &mut Vec<Element>) {
    loop {
        if explode(fish) {
            continue;
        }
        if split(fish) {
            continue;
        }

        break;
    }
}

fn reduce_pairs(fish: &mut Vec<Element>) -> bool {
    // println!("fish: {}", to_string(fish));
    for (i, e) in fish.iter().enumerate() {
        match *e {
            Element::Number(l) => {
                if let Element::Number(r) = fish[i + 1] {
                    fish[i - 1] = Element::Number(3 * l + 2 * r);
                    fish.drain((i)..(i + 3));
                    return true;
                }
            }
            _ => (),
        }
    }

    false
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap().trim().to_string();
    let input2: Vec<&str> = input.split_whitespace().collect();
    let mut fish = input2.iter().fold(Vec::new(), |acc, x| {
        let mut fish = parse(x);
        if !acc.is_empty() {
            fish = add(&acc, &fish);
        }
        reduce(&mut fish);
        fish
    });
    while fish.len() > 1 {
        reduce_pairs(&mut fish);
    }
    println!("Magnitude: {}", to_string(&fish));
}
