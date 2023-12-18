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

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Unknown => Direction::Left,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Unknown => Direction::Right,
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
        let mut diff: i64 = 0;
        if self.x == rhs.x {
            diff = self.y as i64 - rhs.y as i64;
        } else {
            diff = self.x as i64 - rhs.x as i64;
        }
        diff
    }
}

struct Digger {
    direction: Direction,
    position: Position,
    color: String,
}

struct Tile {
    dug: bool,
    color: String,
}

#[derive(Debug, Clone)]
struct Input {
    direction: Direction,
    steps: usize,
    color: String,
}

#[derive(Debug, Clone)]
struct Vertex {
    position: Position,
    direction: Direction,
    steps: usize,
    color: String,
}

#[derive(Debug, Clone)]
struct Ground {
    vertices: Vec<Vertex>,
}

impl Ground {
    pub fn dig(&mut self, direction: Direction, steps: usize, color: String) {
        let current_position = self.vertices.last().unwrap().position;
        let current_direction = self.vertices.last().unwrap().direction;
        // let new_direction = match direction {
        //     Direction::Down => direction,
        //     Direction::Up => direction,
        //     Direction::Unknown => direction,
        //     Direction::Left => current_direction.turn_left(),
        //     Direction::Right => current_direction.turn_right(),
        // };
        let new_position = direction.next_natural_position(&current_position, steps as i32);
        self.vertices.push(Vertex {
            position: new_position,
            direction,
            steps,
            color,
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
        println!("{:?}", instruction);
        ground.dig(
            instruction.direction,
            instruction.steps,
            instruction.color.clone(),
        );
    }
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day18_sample");
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
            let color = parts.next().unwrap();
            Input {
                direction,
                steps,
                color: color.to_string(),
            }
        })
        .collect();
    let mut ground = Ground {
        vertices: vec![Vertex {
            position: Position { x: 0, y: 0 },
            direction: Direction::Unknown,
            steps: 0,
            color: "".to_string(),
        }],
    };
    run_digger(&input, &mut ground);
    for vertex in ground.clone().vertices {
        println!("{:?}", vertex);
    }
    println!("{}", ground.calculate_polygon_area());
}

pub fn part2() {}
