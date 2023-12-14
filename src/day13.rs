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
    // println!("{:?}", pattern);
    let mut temp: Vec<String> = vec!["".to_string(); pattern[0].len()];
    for line in pattern {
        for (i, c) in line.chars().enumerate() {
            temp[i].push_str(&c.to_string());
        }
    }
    // println!("{:?}", temp);
    temp
}

fn find(pattern: &Vec<String>) -> usize {
    let length = pattern.len();
    let mut count = 0;

    // println!("{}", length);
    for i in 0..length - 1 {
        // println!("Searching mirror at {}", i);
        let mut mirror_found = false;
        for (j, k) in (0..=i).rev().zip(i + 1..length) {
            // println!("Compareing {} {} {} {}", j, k, pattern[j], pattern[k]);
            if pattern[j] != pattern[k] {
                mirror_found = false;
                break;
            }
            mirror_found = true;
        }
        if mirror_found {
            count = i + 1;
            break;
        }
    }
    // println!("{}", count);
    count
}

pub fn part1() {
    let patterns = form_data("./inputs/day13");

    let mut remaining_patterns: Vec<Vec<String>> = vec![];

    let mut sum = 0;
    for pattern in patterns.clone() {
        let lines = find(&pattern);
        if lines == 0 {
            remaining_patterns.push(pattern);
        }
        sum += 100 * lines;
    }

    for pattern in remaining_patterns {
        // println!("{:?}", pattern);
        let lines = find(&transpose(&pattern));
        if lines == 0 {
            panic!("remaining 0");
        }
        sum += lines;
    }
    println!("{}", sum);
}

fn compute(pattern: &Vec<String>, lines: usize) -> usize {
    let mut new_lines = 0;
    let mut found = false;
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            let mut new_pattern = pattern.clone();
            if new_pattern[i].chars().nth(j).unwrap() == '#' {
                new_pattern[i].replace_range(j..j + 1, ".");
            } else {
                new_pattern[i].replace_range(j..j + 1, "#");
            }
            // println!("{} {}: {:?}", i, j, new_pattern);
            new_lines = find(&new_pattern);
            if new_lines != 0 && new_lines != lines {
                println!("{} {} {} ", i, j, new_lines);
                found = true;
                break;
            }
        }
        if found {
            break;
        } else {
            new_lines = 0;
        }
    }
    new_lines
}

pub fn part2() {
    let patterns = form_data("./inputs/day13");

    let mut remaining_patterns: Vec<Vec<String>> = vec![];

    let mut sum = 0;
    for pattern in patterns.clone() {
        let mut lines = find(&pattern);
        let mut new_lines;

        if lines != 0 {
            new_lines = compute(&pattern, lines);
            // println!("1H {} {} {:?}", lines, new_lines, pattern.clone());
            sum += 100 * new_lines;
        } else {
            let p = transpose(&pattern);
            lines = find(&p);
            new_lines = 0;
            if lines != 0 {
                new_lines = compute(&p, lines);
            }
            // println!("1V {} {} {:?}", lines, new_lines, pattern);
            sum += new_lines;
        }
        if new_lines == 0 {
            remaining_patterns.push(pattern);
        }
    }

    for p in remaining_patterns.clone() {
        let pattern = transpose(&p);
        let mut lines = find(&pattern);
        let mut new_lines;

        if lines != 0 {
            new_lines = compute(&p, 0);
            // println!("2V {} {} {:?}", lines, new_lines, p.clone());
            sum += 100 * new_lines;
        } else {
            lines = find(&p);
            new_lines = 0;
            if lines != 0 {
                new_lines = compute(&pattern, 0);
            }
            // println!("2H {} {} {:?}", lines, new_lines, p);
            sum += new_lines;
        }
        if new_lines == 0 {
            println!("FOUNd0==========={:?}", p);
        }
    }
    println!("{} {}", patterns.len(), remaining_patterns.len());
    println!("{}", sum);
}
