use crate::utils;

fn form_data(path: &str) -> Vec<Vec<String>> {
    let lines = utils::read_lines(path);
    let mut patterns: Vec<Vec<String>> = Vec::new();
    let mut temp: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            patterns.push(temp.clone());
            temp.clear();
            continue;
        } else {
            temp.push(line);
        }
    }
    patterns.push(temp);
    patterns
}

fn transpose(pattern: &Vec<String>) -> Vec<String> {
    println!("{:?}", pattern);
    let mut temp: Vec<String> = vec!["".to_string(); pattern[0].len()];
    for line in pattern {
        for (i, c) in line.chars().enumerate() {
            temp[i].push_str(&c.to_string());
        }
    }
    println!("{:?}", temp);
    temp
}

fn find(pattern: &Vec<String>) -> usize {
    let length = pattern.len();
    let forward_iter = pattern.iter();
    let mut count = 0;
    for (outer_index, _) in forward_iter.clone().enumerate() {
        let mut mismatch: bool = false;
        let temp_forward = forward_iter.clone().skip(outer_index);
        for (index, temp_line) in temp_forward.enumerate() {
            println!(
                "Comparing {} with {}",
                temp_line,
                pattern[length - 1 - index]
            );
            if *temp_line != pattern[length - 1 - index] {
                mismatch = true;
                break;
            } else if index >= length - 1 - index {
                count = index + outer_index;
                break;
            }
        }
        if !mismatch {
            break;
        }
    }
    count
}

pub fn part1() {
    let patterns = form_data("./inputs/day13_sample");

    let mut remaining_patterns: Vec<Vec<String>> = vec![];

    let mut sum = 0;
    for pattern in patterns {
        println!("{:?}", pattern);
        let lines = find(&pattern);
        if lines == 0 {
            remaining_patterns.push(pattern);
        }
        sum += 100 * lines;
    }

    for pattern in remaining_patterns {
        let lines = find(&transpose(&pattern));
        sum += lines;
    }
    println!("{}", sum);
}

pub fn part2() {}
