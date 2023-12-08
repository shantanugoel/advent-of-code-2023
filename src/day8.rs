use crate::utils;
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
