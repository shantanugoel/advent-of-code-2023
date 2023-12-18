use std::ops::Sub;

use crate::utils;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

impl Direction {
    pub fn next_natural_position(&self, position: &Position, steps: i32) -> Position {
        match self {
            Direction::Up => Position {
                x: position.x,
                y: position.y - steps,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y + steps,
            },
            Direction::Left => Position {
                x: position.x - steps,
                y: position.y,
            },
            Direction::Right => Position {
                x: position.x + steps,
                y: position.y,
            },
            Direction::Unknown => Position {
                x: position.x + steps,
                y: position.y,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Sub for Position {
    type Output = i64;

    fn sub(self, rhs: Self) -> i64 {
        if self.x == rhs.x {
            self.y as i64 - rhs.y as i64
        } else {
            self.x as i64 - rhs.x as i64
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    direction: Direction,
    steps: usize,
}

#[derive(Debug, Clone)]
struct Vertex {
    position: Position,
}

#[derive(Debug, Clone)]
struct Ground {
    vertices: Vec<Vertex>,
}

impl Ground {
    pub fn dig(&mut self, direction: Direction, steps: usize) {
        let current_position = self.vertices.last().unwrap().position;
        let new_position = direction.next_natural_position(&current_position, steps as i32);
        self.vertices.push(Vertex {
            position: new_position,
        });
    }

    pub fn calculate_polygon_area(&self) -> i64 {
        let length = self.vertices.len();
        let mut area = 0;
        for i in 0..length {
            let p1 = self.vertices[i].position;
            let p2 = self.vertices[(i + 1) % length].position;
            area += p1.x as i64 * p2.y as i64 - p2.x as i64 * p1.y as i64 + (p1 - p2).abs();
        }
        area.abs() / 2 + 1
    }
}

fn run_digger(input: &Vec<Input>, ground: &mut Ground) {
    for instruction in input {
        ground.dig(instruction.direction, instruction.steps);
    }
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day18");
    let input: Vec<Input> = lines
        .iter()
        .map(|s| {
            let mut parts = s.split_whitespace();
            let direction = match parts.next().unwrap() {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "D" => Direction::Down,
                "U" => Direction::Up,
                _ => Direction::Unknown,
            };
            let steps = parts.next().unwrap().parse().unwrap();
            Input { direction, steps }
        })
        .collect();
    let mut ground = Ground {
        vertices: vec![Vertex {
            position: Position { x: 0, y: 0 },
        }],
    };
    run_digger(&input, &mut ground);
    println!("{}", ground.calculate_polygon_area());
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day18");
    let input: Vec<Input> = lines
        .iter()
        .map(|s| {
            let parts = s.split('#');
            let direction = match parts.clone().last().unwrap().chars().nth(5).unwrap() {
                '0' => Direction::Right,
                '2' => Direction::Left,
                '1' => Direction::Down,
                '3' => Direction::Up,
                _ => Direction::Unknown,
            };
            let steps =
                u32::from_str_radix(parts.last().unwrap().get(0..5).unwrap(), 16).unwrap() as usize;
            Input { direction, steps }
        })
        .collect();
    let mut ground = Ground {
        vertices: vec![Vertex {
            position: Position { x: 0, y: 0 },
        }],
    };
    run_digger(&input, &mut ground);
    println!("{}", ground.calculate_polygon_area());
}
