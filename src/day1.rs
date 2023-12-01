use std::io::BufRead;

pub fn part1() {
    // read a file called day1
    let file = std::fs::File::open("../inputs/day1").unwrap();
    let mut sum = 0;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        for c in line.chars() {
            if c.is_ascii_digit() {
                sum += c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                sum += c.to_digit(10).unwrap();
                break;
            }
        }
    }
    println!("Day 1 Part 1: {}", sum);
}

fn digit_value(slice: &str) -> u32 {
    let lookup = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut digit = 0;
    // println!("{} {}", slice, slice.len());
    for index in 0..slice.len() {
        for i in 0..index {
            // println!("{} {} {} {}", i, index, slice, slice.len());
            digit = lookup
                .iter()
                .position(|&x| x == &slice[i..index + 1])
                .unwrap_or(0)
                .try_into()
                .unwrap();
            if digit > 0 {
                break;
            }
        }
        if digit > 0 {
            break;
        }
    }
    digit
}

pub fn part2() {
    let file = std::fs::File::open("../inputs/day1").unwrap();
    let mut sum = 0;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        for (i, c) in line.char_indices() {
            let digit;
            if c.is_ascii_digit() {
                digit = c.to_digit(10).unwrap();
            } else {
                // println!("{} {}", i, line);
                digit = digit_value(&line[..i + 1]);
            }
            if digit > 0 {
                // println!("{}", digit);
                sum += digit * 10;
                break;
            }
        }

        let length = line.len() - 1;
        for (i, c) in line.chars().rev().enumerate() {
            let digit;
            if c.is_ascii_digit() {
                digit = c.to_digit(10).unwrap();
            } else {
                // println!("{} {} {}", i, length, line);
                digit = digit_value(&line[(length - i)..]);
            }
            if digit > 0 {
                // println!("{}", digit);
                sum += digit;
                break;
            }
        }
    }
    println!("Day 1 Part 2: {}", sum);
}
