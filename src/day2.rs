use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

pub fn part1() {
    // read a file called day1
    let file = std::fs::File::open("./inputs/day2").unwrap();
    let mut num_games_possible = 0;
    let num_colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let re_colors = Regex::new(r"(\d+) (\w+)").unwrap();
    let re_game_id = Regex::new(r"Game (\d+):").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let data_begin = line.find(": ").unwrap();
        let mut game_possible = true;
        let id = re_game_id.captures(&line[0..data_begin + 1]).unwrap()[1]
            .parse::<i32>()
            .unwrap();
        for cap in re_colors.captures_iter(&line[data_begin + 2..]) {
            let num = cap[1].parse::<i32>().unwrap();
            let color = cap[2].to_string();
            if num > *num_colors.get(&color.as_str()).unwrap() {
                game_possible = false;
                break;
            }
        }
        if game_possible {
            num_games_possible += id;
        }
    }
    println!("Number of possible games: {}", num_games_possible);
}

pub fn part2() {
    let file = std::fs::File::open("./inputs/day2").unwrap();
    let re_colors = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut power = 0;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let data_begin = line.find(": ").unwrap();
        let mut min_red = 1;
        let mut min_green = 1;
        let mut min_blue = 1;
        for cap in re_colors.captures_iter(&line[data_begin + 2..]) {
            let num = cap[1].parse::<i32>().unwrap();
            let color = cap[2].to_string();
            match color.as_str() {
                "red" => {
                    if num > min_red {
                        min_red = num;
                    }
                }
                "green" => {
                    if num > min_green {
                        min_green = num;
                    }
                }
                "blue" => {
                    if num > min_blue {
                        min_blue = num;
                    }
                }
                _ => {
                    println!("Unknown color: {}", color);
                }
            }
        }
        power += min_red * min_green * min_blue;
    }
    println!("Power: {}", power);
}
