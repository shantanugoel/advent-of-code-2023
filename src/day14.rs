use core::panic;

use crate::utils;

fn transpose(pattern: &Vec<String>) -> Vec<String> {
    let mut temp: Vec<String> = vec!["".to_string(); pattern[0].len()];
    for line in pattern {
        for (i, c) in line.chars().enumerate() {
            temp[i].push_str(&c.to_string());
        }
    }
    temp
}

fn get_input(path: &str) -> Vec<String> {
    let input = utils::read_lines(path);
    // transpose(&input)
    input
}

pub fn part1() {
    let lines = get_input("./inputs/day14_sample");
    let length = lines[0].len();
    let mut weight = 0;
    let mut spaces: Vec<usize> = vec![];
    for line in lines {
        // println!("{}", line);
        spaces.clear();
        for (i, c) in line.chars().enumerate() {
            // println!("{}: {}", i, c);
            match c {
                'O' => {
                    if spaces.is_empty() {
                        weight += length - i;
                    } else {
                        // println!("Taking up space {}", spaces[0]);
                        weight += spaces[0];
                        spaces.remove(0);
                        spaces.push(length - i);
                    }
                }
                '.' => spaces.push(length - i),
                '#' => spaces.clear(),
                _ => continue,
            }
        }
        // println!("{}: weight {}", line, weight);
    }
    println!("weight {}", weight);
}

#[derive(PartialEq, Eq)]
enum FlipDirection {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn flip(input: &Vec<String>, direction: FlipDirection) -> Vec<String> {
    if direction == FlipDirection::Horizontal {
        input
            .iter()
            .map(|line| line.chars().rev().collect())
            .collect()
    } else {
        input.iter().rev().cloned().collect()
    }
}

fn tilt(input: &Vec<String>, direction: Direction) -> Vec<String> {
    match direction {
        Direction::North => transpose(input),
        Direction::West => flip(input, FlipDirection::Horizontal),
        Direction::East => flip(input, FlipDirection::Vertical),
        Direction::South => flip(&transpose(input), FlipDirection::Horizontal),
    }
}

fn slide(input: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = input.clone();
    // println!("{:?}", input);
    for line in output.iter_mut() {
        let mut beginning = 0;
        for (i, c) in line.clone().chars().enumerate() {
            // println!("{}: {}", i, c);
            match c {
                '#' => {
                    beginning = i + 1;
                }
                '.' => continue,
                'O' => {
                    if beginning != i {
                        // println!("---------{} {}", beginning, i);
                        // println!("{}", line);
                        line.replace_range(beginning..beginning + 1, "O");
                        line.replace_range(i..i + 1, ".");
                        // println!("{}", line);
                    }
                    beginning += 1;
                }
                _ => {}
            }
        }
    }
    output
}

fn cycle(input: &Vec<String>, loop_count: usize) -> Vec<String> {
    let mut output: Vec<String> = input.clone();
    for i in 0..loop_count {
        if i % 100000 == 0 {
            println!("{}", i);
        }
        output = transpose(&slide(&transpose(&output)));
        // println!("{:?}", output);
        output = slide(&output);
        // println!("{:?}", output);
        // println!("{:?}", output);
        output = transpose(&flip(
            &slide(&flip(&transpose(&output), FlipDirection::Horizontal)),
            FlipDirection::Horizontal,
        ));
        output = flip(
            &slide(&flip(&output, FlipDirection::Horizontal)),
            FlipDirection::Horizontal,
        );
        // println!("{:?}", output);
    }
    output
}

fn compute(input: &Vec<String>) -> usize {
    let length = input[0].len();
    let mut weight = 0;
    let mut spaces: Vec<usize> = vec![];
    for line in input {
        spaces.clear();
        for (i, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    if spaces.is_empty() {
                        weight += length - i;
                    } else {
                        weight += spaces[0];
                        spaces.remove(0);
                        spaces.push(length - i);
                    }
                }
                '.' => spaces.push(length - i),
                '#' => spaces.clear(),
                _ => continue,
            }
        }
    }
    weight
}
pub fn part2() {
    let lines = get_input("./inputs/day14_sample");
    // let weight = compute(&cycle(&lines, 1000000000));
    // 10000000
    // 1000000000
    let weight = compute(&transpose(&cycle(&lines, 10)));
    println!("weight {}", weight);
}
