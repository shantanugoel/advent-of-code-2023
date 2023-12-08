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
