use crate::utils;

#[derive(Debug, Clone, Copy)]
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

pub fn part2() {}
