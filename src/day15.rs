use crate::utils;

fn hash(input: &str, initial_value: u64) -> u64 {
    let mut hash = initial_value;
    for c in input.chars() {
        hash += (c as u8) as u64;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn compute(input: &str) -> u64 {
    input.split(',').fold(0, |result, s| result + hash(s, 0))
}

pub fn part1() {
    let input = utils::read_lines("./inputs/day15");
    println!("{}", compute(input[0].as_str()));
}

#[derive(Clone)]
struct Lens {
    label: String,
    focus: u32,
}

pub fn part2() {
    let input = utils::read_lines("./inputs/day15");
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    for lens in input[0].split(',') {
        if let Some((label, focus)) = lens.split_once('=') {
            let mut found = false;
            let hash = hash(label, 0);
            for lens in boxes[hash as usize].iter_mut() {
                if lens.label == label {
                    lens.focus = focus.parse().unwrap();
                    found = true;
                    break;
                }
            }
            if !found {
                boxes[hash as usize].push(Lens {
                    label: label.to_string(),
                    focus: focus.parse().unwrap(),
                });
            }
        } else {
            let label = &lens[..lens.len() - 1];
            let hash = hash(label, 0);
            boxes[hash as usize].retain(|lens| lens.label != label);
        }
    }

    let mut power = 0;
    for (index, boxx) in boxes.iter().enumerate() {
        for (slot, lens) in boxx.iter().enumerate() {
            power += (index as u32 + 1) * (slot as u32 + 1) * lens.focus;
        }
    }
    println!("{}", power);
}
