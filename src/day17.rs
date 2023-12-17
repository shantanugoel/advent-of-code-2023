use std::collections::{HashMap, VecDeque};

use crate::utils;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum LavaDirection {
    Up,
    Down,
    Left,
    Right,
}

impl LavaDirection {
    pub fn next_natural_position(&self, position: &Position) -> Position {
        match self {
            LavaDirection::Up => Position {
                x: position.x,
                y: position.y - 1,
            },
            LavaDirection::Down => Position {
                x: position.x,
                y: position.y + 1,
            },
            LavaDirection::Left => Position {
                x: position.x - 1,
                y: position.y,
            },
            LavaDirection::Right => Position {
                x: position.x + 1,
                y: position.y,
            },
        }
    }

    pub fn turn_left(&self) -> LavaDirection {
        match self {
            LavaDirection::Up => LavaDirection::Left,
            LavaDirection::Left => LavaDirection::Down,
            LavaDirection::Down => LavaDirection::Right,
            LavaDirection::Right => LavaDirection::Up,
        }
    }

    pub fn turn_right(&self) -> LavaDirection {
        match self {
            LavaDirection::Up => LavaDirection::Right,
            LavaDirection::Right => LavaDirection::Down,
            LavaDirection::Down => LavaDirection::Left,
            LavaDirection::Left => LavaDirection::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn is_valid(&self, width: usize, height: usize) -> bool {
        // println!(
        //     "Position::is_valid: {}, {}, {}, {}",
        //     self.x, self.y, width, height
        // );
        self.x < width && self.y < height && self.x as i32 >= 0 && self.y as i32 >= 0
    }

    pub fn reached_factory(&self, width: usize, height: usize) -> bool {
        self.x == width - 1 && self.y == height - 1
        // self.x == 4 && self.y == 1
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Lava {
    direction: LavaDirection,
    position: Position,
    straight_moved: usize,
    heat_loss: usize,
    valid: bool,
}

impl Lava {
    pub fn new(
        direction: LavaDirection,
        position: Position,
        straight_moved: usize,
        heat_loss: usize,
    ) -> Lava {
        // println!("=============>New Lava");
        Lava {
            direction,
            position,
            straight_moved,
            heat_loss,
            valid: true,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn move_forward(&mut self) -> Vec<Lava> {
        let mut lavas: Vec<Lava> = vec![];
        let mut lava = Lava::new(self.direction.turn_left(), self.position, 0, self.heat_loss);
        lava.position = lava.direction.next_natural_position(&self.position);
        lavas.push(lava);
        lava = Lava::new(
            self.direction.turn_right(),
            self.position,
            0,
            self.heat_loss,
        );
        lava.position = lava.direction.next_natural_position(&self.position);
        lavas.push(lava);
        if self.straight_moved < 2 {
            self.straight_moved += 1;
            self.position = self.direction.next_natural_position(&self.position);
        } else {
            self.valid = false;
        }
        lavas
    }
}

fn traverse(
    input: Vec<Vec<usize>>,
    initial_lava: Lava,
    losses: &mut Vec<usize>,
    existing_paths: &mut HashMap<(LavaDirection, Position, usize), usize>,
) {
    let width = input[0].len();
    let height = input.len();
    // println!("New Lava {:?}", initial_lava);
    // let mut new_lava = initial_lava;
    let mut lava_queue: VecDeque<Lava> = VecDeque::from(vec![initial_lava]);

    while let Some(mut new_lava) = lava_queue.pop_front() {
        while new_lava.is_valid()
            && new_lava.position.is_valid(width, height)
            && !new_lava.position.reached_factory(width, height)
        {
            println!("{}", lava_queue.len());
            // println!("Loop lava {:?}", new_lava);
            let key = (
                new_lava.direction,
                new_lava.position,
                new_lava.straight_moved,
            );
            if existing_paths.contains_key(&key)
                && *existing_paths.get(&key).unwrap() <= new_lava.heat_loss
            {
                break;
            } else {
                existing_paths.insert(key, new_lava.heat_loss);
            }
            // new_lava.heat_loss += input[new_lava.position.y][new_lava.position.x];
            let mut lavas = new_lava.move_forward();
            if new_lava.position.is_valid(width, height) {
                new_lava.heat_loss += input[new_lava.position.y][new_lava.position.x];
            }
            for lava in lavas.iter_mut() {
                if lava.position.is_valid(width, height) {
                    lava.heat_loss += input[lava.position.y][lava.position.x];
                    lava_queue.push_back(lava.clone());
                    // traverse(input.clone(), lava.clone(), losses, existing_paths);
                }
            }
        }
        if new_lava.position.reached_factory(width, height) {
            // println!("Adding Loss {} to {:?}", initial_lava.heat_loss, losses);
            losses.push(new_lava.heat_loss);
        }
    }
}

pub fn part1() {
    let mut input: Vec<Vec<usize>> = utils::read_lines("./inputs/day17")
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    input[0][0] = 0;
    let mut losses: Vec<usize> = vec![];
    let initial_lava = Lava::new(LavaDirection::Right, Position { x: 0, y: 0 }, 0, 0);
    let mut existing_paths: HashMap<(LavaDirection, Position, usize), usize> = HashMap::new();
    traverse(input, initial_lava, &mut losses, &mut existing_paths);
    println!("{:?}", losses);
    println!("{}", losses.iter().min().unwrap());
}

pub fn part2() {}
