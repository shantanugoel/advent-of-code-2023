use crate::utils;
use regex::Regex;

pub fn part1() {
    let lines = utils::read_lines("./inputs/day4");
    let re = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;
    for line in lines {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_str, my_str) = numbers.split_once('|').unwrap();
        let winning_numbers: Vec<i32> = re
            .find_iter(winning_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let my_numbers: Vec<i32> = re
            .find_iter(my_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let mut prod = 0;
        for num in my_numbers {
            if winning_numbers.contains(&num) {
                if prod == 0 {
                    prod = 1;
                } else {
                    prod *= 2;
                }
            }
        }
        sum += prod;
    }
    println!("Day 4 part 1: {}", sum);
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day4");
    let re = Regex::new(r"(\d+)").unwrap();
    let lines_len = lines.len();
    let mut card_counter: Vec<i32> = vec![1; lines_len];
    for (index, line) in lines.into_iter().enumerate() {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_str, my_str) = numbers.split_once('|').unwrap();
        let winning_numbers: Vec<i32> = re
            .find_iter(winning_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let my_numbers: Vec<i32> = re
            .find_iter(my_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let mut temp_index = index;
        for num in my_numbers {
            if winning_numbers.contains(&num) {
                temp_index += 1;
                if temp_index < lines_len {
                    card_counter[temp_index] += card_counter[index];
                } else {
                    break;
                }
            }
        }
    }
    let sum: i32 = card_counter.iter().sum();
    println!("Day 4 part 2: {}", sum);
}
