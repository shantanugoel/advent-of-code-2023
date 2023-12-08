use crate::utils;
use num_integer::lcm;
use std::collections::HashMap;

pub fn part1() {
    let lines = utils::read_lines("./inputs/day8");
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.iter().skip(2) {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        map.insert(key, (left, right));
    }

    let mut current = map.get("AAA").unwrap();
    let mut steps = 0;
    for instruction in lines[0].chars().cycle() {
        let key = if instruction == 'L' {
            current.0
        } else {
            current.1
        };
        steps += 1;
        if key == "ZZZ" {
            break;
        }
        current = map.get(key).unwrap();
    }
    println!("{}", steps);
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day8");
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.iter().skip(2) {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        map.insert(key, (left, right));
    }

    let mut currents: Vec<&(&str, &str)> = map
        .iter()
        .filter_map(|(key, value)| {
            if key.ends_with('A') {
                Some(value)
            } else {
                None
            }
        })
        .collect();
    let mut steps: u64 = 0;
    let mut first_z_steps: Vec<u64> = vec![0; currents.len()];
    for instruction in lines[0].chars().cycle() {
        let mut keys = vec![];
        for current in currents.clone() {
            if instruction == 'L' {
                keys.push(current.0);
            } else {
                keys.push(current.1);
            }
        }
        steps += 1;
        for i in 0..first_z_steps.len() {
            if first_z_steps[i] == 0 && keys[i].ends_with('Z') {
                first_z_steps[i] = steps;
            }
        }
        if first_z_steps.iter().all(|&x| x != 0) {
            break;
        }
        currents.clear();
        for key in keys {
            currents.push(map.get(key).unwrap());
        }
    }

    println!("{}", first_z_steps.iter().cloned().fold(1, lcm));
}
