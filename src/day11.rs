use crate::utils;

fn expand(map: &Vec<Vec<char>>) -> Vec<Vec<u32>> {
    let mut expanded_map: Vec<Vec<u32>> = vec![];
    let mut galaxy_count = 0;
    for row in map {
        let mut new_row = vec![0; row.len()];
        let mut empty_row = true;
        for col_index in 0..map[0].len() {
            if row[col_index] != '.' {
                empty_row = false;
                galaxy_count += 1;
                new_row[col_index] = galaxy_count;
            }
        }
        expanded_map.push(new_row.clone());
        if empty_row {
            expanded_map.push(new_row);
        }
    }

    let mut offset = 0;
    for col_index in 0..map[0].len() {
        let mut empty_column = true;
        for row in expanded_map.iter() {
            if row[col_index + offset] != 0 {
                empty_column = false;
            }
        }
        if empty_column {
            offset += 1;
            for row in expanded_map.iter_mut() {
                row.insert(col_index + offset, 0);
            }
        }
    }
    expanded_map
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day11_sample");
    let map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let expanded_map = expand(&map);
}

pub fn part2() {}
