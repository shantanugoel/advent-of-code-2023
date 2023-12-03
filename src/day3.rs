use crate::utils;

fn has_symbol(line: &str) -> bool {
    let mut has_symbol = true;
    let re = regex::Regex::new(r"[^.0-9]").unwrap();
    if re.captures(line).is_none() {
        has_symbol = false;
    }
    has_symbol
}

fn is_symbol(c: char) -> bool {
    let mut result = true;
    if c.is_ascii_digit() || c == '.' {
        result = false;
    }
    result
}
pub fn part1() {
    let lines = utils::read_lines("./inputs/day3");
    let re = regex::Regex::new(r"([0-9]+)").unwrap();
    let mut sum_parts = 0;
    for (index, line) in lines.clone().into_iter().enumerate() {
        // Find numbers in the current line along with their positions
        let numbers = re.find_iter(&line);
        for matches in numbers {
            let number = matches.as_str().parse::<i32>().unwrap();
            let start = matches.start();
            let end = matches.end() - 1;
            let mut start_diagonal = start;
            if start != 0 {
                start_diagonal -= 1;
            }
            let mut end_diagonal = end;
            if end < line.len() - 2 {
                end_diagonal += 2;
            }

            // check current line if it has a symbol before or after the number position
            if start != 0 && is_symbol(line.chars().nth(start - 1).unwrap()) {
                sum_parts += number;
                continue;
            }
            if end < line.len() - 1 && is_symbol(line.chars().nth(end + 1).unwrap()) {
                sum_parts += number;
                continue;
            }

            // Check previous line if it has a symbol in the range of the number
            if index != 0 {
                let prev_line = lines.get(index - 1).unwrap();
                if has_symbol(&prev_line[start_diagonal..end_diagonal]) {
                    sum_parts += number;
                    continue;
                }
            }

            // check next line if it has a symbol in the range of the number
            if index != lines.len() {
                let next_line = lines.get(index + 1).unwrap();
                if has_symbol(&next_line[start_diagonal..end_diagonal]) {
                    sum_parts += number;
                    continue;
                }
            }
        }
    }
    println!("Sum of parts: {}", sum_parts);
}

#[derive(Debug, Clone)]
struct Numbers {
    start: usize,
    end: usize,
    number: i32,
}

fn get_touch_numbers(numbers: Vec<Numbers>, index: usize, diag: bool) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for number in numbers.clone() {
        if (number.start != 0 && index == number.start - 1) || index == number.end + 1 {
            result.push(number.number);
            continue;
        }
        if diag && (index >= number.start && index <= number.end) {
            result.push(number.number);
        }
    }
    // println!("{:?}: {}: {:?}", numbers, index, result);
    result
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day3");
    let re = regex::Regex::new(r"([0-9]+)").unwrap();
    let mut parsed_numbers: Vec<Vec<Numbers>> = Vec::new();
    let mut star_positions: Vec<Vec<usize>> = Vec::new();
    let lines_len = lines.len();
    let mut sum = 0;
    for line in lines {
        let mut temp: Vec<Numbers> = Vec::new();
        let numbers = re.find_iter(&line);
        for matches in numbers {
            let number = matches.as_str().parse::<i32>().unwrap();
            let start = matches.start();
            let end = matches.end() - 1;
            temp.push(Numbers { start, end, number });
        }
        parsed_numbers.push(temp);
        let star_temp: Vec<usize> = line
            .char_indices()
            .filter(|&(_, c)| c == '*')
            .map(|(pos, _)| pos)
            .collect();
        star_positions.push(star_temp);
    }
    for (index, stars) in star_positions.into_iter().enumerate() {
        for star in stars {
            let mut touching_numbers: Vec<i32> = Vec::new();
            if index != 0 {
                //search in prev line
                touching_numbers.append(&mut get_touch_numbers(
                    parsed_numbers.get(index - 1).unwrap().to_vec(),
                    star,
                    true,
                ));
            }
            // Search in current line
            touching_numbers.append(&mut get_touch_numbers(
                parsed_numbers.get(index).unwrap().to_vec(),
                star,
                false,
            ));
            if index < lines_len - 1 {
                // Search in next line
                touching_numbers.append(&mut get_touch_numbers(
                    parsed_numbers.get(index + 1).unwrap().to_vec(),
                    star,
                    true,
                ));
            }
            if touching_numbers.len() == 2 {
                sum += touching_numbers[0] * touching_numbers[1];
            }
        }
    }
    println!("Sum: {}", sum);
}
