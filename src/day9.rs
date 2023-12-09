use crate::utils;

fn predict(history: Vec<i32>, part: i32) -> i32 {
    let mut next: Vec<i32> = vec![];
    let predicted = if history.len() == 1 {
        history[0]
    } else if history.iter().any(|&v| v != 0) {
        for window in history.windows(2) {
            next.push(window[1] - window[0]);
        }
        if !next.is_empty() {
            if part == 1 {
                history.last().unwrap() + predict(next, part)
            } else {
                history.first().unwrap() - predict(next, part)
            }
        } else {
            0
        }
    } else {
        0
    };
    predicted
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day9");
    let mut predicted: Vec<i32> = vec![];
    for line in lines {
        let history: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        predicted.push(predict(history, 1));
    }
    println!("{}", predicted.iter().sum::<i32>());
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day9");
    let mut predicted: Vec<i32> = vec![];
    for line in lines {
        let history: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        predicted.push(predict(history, 2));
    }
    println!("{}", predicted.iter().sum::<i32>());
}
