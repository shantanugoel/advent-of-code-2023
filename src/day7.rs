use crate::utils;
use phf::phf_map;
use std::collections::BTreeMap;

static REMAP: phf::Map<char, u32> = phf_map! {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 11,
    'T' => 10,
    '9' => 9,
    '8' => 8,
    '7' => 7,
    '6' => 6,
    '5' => 5,
    '4' => 4,
    '3' => 3,
    '2' => 2,
};

static REMAP2: phf::Map<char, u32> = phf_map! {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'T' => 10,
    '9' => 9,
    '8' => 8,
    '7' => 7,
    '6' => 6,
    '5' => 5,
    '4' => 4,
    '3' => 3,
    '2' => 2,
    'J' => 1,
};

fn remap_score(line: &str) -> Vec<u32> {
    let mut data: Vec<u32> = vec![0];
    for c in line.chars() {
        data.push(*REMAP.get(&c).unwrap());
    }
    let mut score_array = [0, 1, 0, 0, 0, 0];
    for i in 2..6 {
        let mut found = false;
        for j in 1..i {
            if data[j] == data[i] {
                score_array[j] *= 3;
                found = true;
                break;
            }
        }
        if !found {
            score_array[i] = 1;
        }
    }
    for score in score_array {
        data[0] += score;
    }
    data
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day7");
    let mut sorted_input: BTreeMap<Vec<u32>, u32> = BTreeMap::new();
    for line in lines {
        let input: Vec<&str> = line.split_whitespace().collect();
        let bid: u32 = input[1].parse().unwrap();
        let remapped_input = remap_score(input[0]);
        sorted_input.insert(remapped_input, bid);
    }
    let mut sum = 0;
    for (index, value) in sorted_input.values().enumerate() {
        sum += value * (index as u32 + 1);
    }
    println!("{}", sum);
}

fn remap_score2(line: &str) -> Vec<u32> {
    let mut data: Vec<u32> = vec![0];
    for c in line.chars() {
        data.push(*REMAP2.get(&c).unwrap());
    }
    let mut score_array = [0, 0, 0, 0, 0, 0];
    for i in 1..6 {
        if data[i] != 1 {
            let mut found = false;
            for j in 1..i {
                if data[j] == data[i] {
                    score_array[j] *= 3;
                    found = true;
                    break;
                }
            }
            if !found {
                score_array[i] = 1;
            }
        }
    }
    let (max_index, &_) = score_array
        .iter()
        .enumerate()
        .max_by_key(|&(_, value)| value)
        .unwrap();

    for value in data.iter().skip(1) {
        if *value == 1 {
            score_array[max_index] *= 3;
        }
    }
    if line == "JJJJJ" {
        score_array[max_index] = 81;
    }

    for index in 1..6 {
        if data[index] == 1 && score_array[index] == 1 {
            score_array[index] = 0;
        }
    }

    // Check for full house
    let mut house_partial_1 = false;
    let mut house_partial_2 = false;
    for score in score_array {
        if score == 3 {
            house_partial_1 = true;
            continue;
        }
        if score == 9 {
            house_partial_2 = true;
            continue;
        }
    }
    if house_partial_1 && house_partial_2 {
        score_array[max_index] *= 2;
    }
    for score in score_array {
        data[0] += score;
    }
    data
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day7");
    let mut sorted_input: BTreeMap<Vec<u32>, (u32, &str)> = BTreeMap::new();
    let lines2 = lines.clone();
    for (index, line) in lines.into_iter().enumerate() {
        let input: Vec<&str> = line.split_whitespace().collect();
        let bid: u32 = input[1].parse().unwrap();
        let remapped_input = remap_score2(input[0]);
        sorted_input.insert(remapped_input, (bid, &lines2[index]));
    }
    let mut sum = 0;
    for (index, value) in sorted_input.values().enumerate() {
        sum += value.0 * (index as u32 + 1);
    }
    println!("{}", sum);
}
