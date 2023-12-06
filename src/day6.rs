use crate::utils;

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

fn parse_line_2(line: &str) -> u64 {
    let line = line.replace(' ', "");
    line.split(':').collect::<Vec<&str>>()[1].parse().unwrap()
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day6");
    let times = parse_line(lines[0].as_str());
    let distances = parse_line(lines[1].as_str());
    let mut margin = 1;
    for (index, distance) in distances.into_iter().enumerate() {
        let mut num_ways = 0;
        for time in 0..times[index] {
            if time * (times[index] - time) > distance {
                num_ways += 1;
            }
        }
        margin *= num_ways;
    }
    println!("{}", margin);
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day6");
    let time = parse_line_2(lines[0].as_str());
    let distance = parse_line_2(lines[1].as_str());
    let mut num_ways = 0;
    for current_time in 0..time {
        if current_time * (time - current_time) > distance {
            num_ways += 1;
        }
    }
    println!("{} {} {}", time, distance, num_ways);
}
