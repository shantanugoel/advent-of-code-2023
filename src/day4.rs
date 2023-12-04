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
