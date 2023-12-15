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
    transpose(&input)
}

pub fn part1() {
    let lines = get_input("./inputs/day14");
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
    // println!("weight {}", weight);
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
        Direction::South => transpose(&flip(input, FlipDirection::Horizontal)),
    }
}

fn cycle(input: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String>;
    output = tilt(input, Direction::North);
    println!("{:?}", output);
    output = tilt(&output, Direction::West);
    println!("{:?}", output);
    output = tilt(&output, Direction::South);
    println!("{:?}", output);
    output = tilt(&output, Direction::East);
    println!("{:?}", output);
    output
}

fn compute(input: &Vec<String>) -> usize {
    let length = input[0].len();
    let mut weight = 0;
    let mut spaces: Vec<usize> = vec![];
    for line in input {
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
    weight
}
pub fn part2() {
    let lines = get_input("./inputs/day14");
    let mut weight = 0;
    // for i in 0..1000000000 {
    for i in 0..1 {
        println!("{}: {}", i, weight);
        weight = compute(&cycle(&lines));
    }
    println!("weight {}", weight);
}
