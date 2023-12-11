use num_integer::Integer;
use plotters::prelude::*;

use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl std::ops::Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    pub fn is_valid(&self, max_x: i32, max_y: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < max_x && self.y < max_y
    }

    pub fn process(&self, other: &Coordinate, pipe: char) -> Option<Coordinate> {
        let mut new_x = other.x;
        let mut new_y = other.y;

        match pipe {
            '|' => {
                if self.x == other.x && (other.y - self.y).abs() == 1 {
                    new_y += other.y - self.y;
                }
            }
            '-' => {
                if (other.x - self.x).abs() == 1 && self.y == other.y {
                    new_x += other.x - self.x;
                }
            }
            'L' => {
                if self.x == other.x && self.y == other.y - 1 {
                    new_x += 1;
                } else if self.x == other.x + 1 && self.y == other.y {
                    new_y -= 1;
                }
            }
            'J' => {
                if self.x == other.x - 1 && self.y == other.y {
                    new_y -= 1;
                } else if self.x == other.x && self.y == other.y - 1 {
                    new_x -= 1;
                }
            }
            '7' => {
                if self.x == other.x && self.y == other.y + 1 {
                    new_x -= 1;
                } else if self.x == other.x - 1 && self.y == other.y {
                    new_y += 1;
                }
            }
            'F' => {
                if self.x == other.x && self.y == other.y + 1 {
                    new_x += 1;
                } else if self.x == other.x + 1 && self.y == other.y {
                    new_y += 1;
                }
            }
            _ => {}
        }

        if new_x != other.x || new_y != other.y {
            Some(Coordinate { x: new_x, y: new_y })
        } else {
            None
        }
    }
}

fn traverse_pipes(map: &Vec<Vec<char>>, start: Coordinate) -> Option<usize> {
    let mut num_steps = 0;
    let mut loop_found = false;
    let directions_to_process = [
        Coordinate { x: 0, y: 1 },
        Coordinate { x: 0, y: -1 },
        Coordinate { x: 1, y: 0 },
        Coordinate { x: -1, y: 0 },
    ];
    for direction in directions_to_process.iter() {
        let mut current = start;
        let mut next = start + *direction;
        num_steps = 1;

        while next.is_valid(map[0].len() as i32, map.len() as i32) {
            // println!(
            //     "{}: {:?}: {}",
            //     num_steps, next, map[next.y as usize][next.x as usize]
            // );
            if map[next.y as usize][next.x as usize] == 'S' {
                loop_found = true;
                break;
            } else {
                num_steps += 1;
                match current.process(&next, map[next.y as usize][next.x as usize]) {
                    Some(coordinate) => {
                        current = next;
                        next = coordinate;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        if loop_found {
            break;
        }
    }
    if loop_found {
        Some(num_steps)
    } else {
        None
    }
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day10");
    let map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    for (row_index, row) in map.iter().enumerate() {
        for (col_index, &character) in row.iter().enumerate() {
            if character == 'S' {
                match traverse_pipes(&map, Coordinate::new(col_index as i32, row_index as i32)) {
                    Some(steps) => {
                        println!("Part 1: {}", steps / 2);
                    }
                    None => {
                        continue;
                    }
                }
                break;
            }
        }
    }
}

fn traverse_pipes_2(map: &Vec<Vec<char>>, start: Coordinate) -> Option<(usize, Vec<Coordinate>)> {
    let mut num_steps = 0;
    let mut loop_found = false;
    let directions_to_process = [
        Coordinate { x: 0, y: 1 },
        Coordinate { x: 0, y: -1 },
        Coordinate { x: 1, y: 0 },
        Coordinate { x: -1, y: 0 },
    ];
    let mut set: Vec<Coordinate> = vec![];
    for direction in directions_to_process.iter() {
        let mut current = start;
        let mut next = start + *direction;
        num_steps = 1;
        set.clear();
        set.push(current);

        while next.is_valid(map[0].len() as i32, map.len() as i32) {
            // println!(
            //     "{}: {:?}: {}",
            //     num_steps, next, map[next.y as usize][next.x as usize]
            // );
            set.push(next);
            if map[next.y as usize][next.x as usize] == 'S' {
                loop_found = true;
                break;
            } else {
                num_steps += 1;
                match current.process(&next, map[next.y as usize][next.x as usize]) {
                    Some(coordinate) => {
                        current = next;
                        next = coordinate;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        if loop_found {
            break;
        }
    }
    if loop_found {
        Some((num_steps, set))
    } else {
        None
    }
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day10");
    let mut map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut loop_set: Vec<Coordinate> = vec![];

    for (row_index, row) in map.iter().enumerate() {
        for (col_index, &character) in row.iter().enumerate() {
            if character == 'S' {
                match traverse_pipes_2(&map, Coordinate::new(col_index as i32, row_index as i32)) {
                    Some((_, set)) => {
                        loop_set = set;
                    }
                    None => {
                        continue;
                    }
                }
                break;
            }
        }
    }

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !loop_set.contains(&Coordinate {
                x: x as i32,
                y: y as i32,
            }) {
                map[y][x] = '.';
            }
        }
    }

    const OUT_FILE_NAME: &str = "animation.gif";
    let root = BitMapBackend::gif(OUT_FILE_NAME, (800, 800), 1)
        .unwrap()
        .into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0..map[0].len(), map.len()..0)
        .unwrap();
    let mut series: Vec<(usize, usize)> = vec![];
    root.fill(&WHITE).unwrap();
    for point in loop_set {
        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()
            .unwrap();
        series.push((point.x as usize, point.y as usize));
        chart
            .draw_series(LineSeries::new(series.iter().cloned(), &RED))
            .unwrap();
        root.present().unwrap();
    }
    // for (y_index, y) in map.iter().enumerate() {
    //     for (x_index, x) in y.iter().enumerate() {
    //         series.push()
    //     }
    // }

    let mut sum = 0;

    for row in map.iter().skip(1).take(map.len() - 2) {
        for (index, &c) in row.iter().skip(1).enumerate() {
            if c == '.' {
                let mut intersection_count = 0;
                let mut flag_f = false;
                let mut flag_l = false;
                for x in row.iter().skip(index + 1) {
                    match *x {
                        'F' => {
                            if flag_f || flag_l {
                                flag_l = false;
                            }
                            flag_f = true;
                        }
                        '7' => {
                            if flag_l {
                                intersection_count += 1;
                            }
                            flag_f = false;
                            flag_l = false;
                        }
                        'L' => {
                            if flag_f || flag_l {
                                flag_f = false;
                            }
                            flag_l = true;
                        }
                        'J' => {
                            if flag_f {
                                intersection_count += 1;
                            }
                            flag_l = false;
                            flag_f = false;
                        }
                        '|' => {
                            if flag_f || flag_l {
                                flag_l = false;
                                flag_f = false;
                            }
                            intersection_count += 1;
                        }
                        'S' => {
                            if flag_f || flag_l {
                                flag_l = false;
                                flag_f = false;
                            }
                        }
                        _ => continue,
                    }
                }
                if intersection_count.is_odd() {
                    sum += 1;
                }
            }
        }
    }
    println!("{}", sum);
}
