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
        println!("{}", line);
        spaces.clear();
        for (i, c) in line.chars().enumerate() {
            println!("{}: {}", i, c);
            match c {
                'O' => {
                    if spaces.is_empty() {
                        weight += length - i;
                    } else {
                        println!("Taking up space {}", spaces[0]);
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
        println!("{}: weight {}", line, weight);
    }
    println!("weight {}", weight);
}

pub fn part2() {}
