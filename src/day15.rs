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

pub fn part2() {}
