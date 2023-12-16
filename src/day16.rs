use std::collections::HashSet;

use crate::utils;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum BeamDirection {
    Up,
    Down,
    Left,
    Right,
}

impl BeamDirection {
    pub fn next_natural_position(&self, position: &Position) -> Position {
        match self {
            BeamDirection::Up => Position {
                x: position.x,
                y: position.y - 1,
            },
            BeamDirection::Down => Position {
                x: position.x,
                y: position.y + 1,
            },
            BeamDirection::Left => Position {
                x: position.x - 1,
                y: position.y,
            },
            BeamDirection::Right => Position {
                x: position.x + 1,
                y: position.y,
            },
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
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Beam {
    direction: BeamDirection,
    position: Position,
}

impl Beam {
    pub fn new(direction: BeamDirection, position: Position) -> Beam {
        // println!("=============>New Beam");
        Beam {
            direction,
            position,
        }
    }

    pub fn move_forward(&mut self, tile: char) -> Option<Beam> {
        let mut new_beam = None;
        match tile {
            '|' => {
                if self.direction == BeamDirection::Up || self.direction == BeamDirection::Down {
                    self.position = self.direction.next_natural_position(&self.position);
                } else if self.direction == BeamDirection::Left
                    || self.direction == BeamDirection::Right
                {
                    self.direction = BeamDirection::Up;
                    self.position = self.direction.next_natural_position(&self.position);
                    let mut temp = Beam::new(BeamDirection::Down, self.position);
                    temp.position = temp.direction.next_natural_position(&temp.position);
                    new_beam = Some(temp);
                }
            }
            '-' => {
                if self.direction == BeamDirection::Left || self.direction == BeamDirection::Right {
                    self.position = self.direction.next_natural_position(&self.position);
                } else if self.direction == BeamDirection::Up
                    || self.direction == BeamDirection::Down
                {
                    self.direction = BeamDirection::Left;
                    self.position = self.direction.next_natural_position(&self.position);
                    let mut temp = Beam::new(BeamDirection::Right, self.position);
                    temp.position = temp.direction.next_natural_position(&temp.position);
                    new_beam = Some(temp);
                }
            }
            '/' => {
                match self.direction {
                    BeamDirection::Up => {
                        self.direction = BeamDirection::Right;
                    }
                    BeamDirection::Down => {
                        self.direction = BeamDirection::Left;
                    }
                    BeamDirection::Left => {
                        self.direction = BeamDirection::Down;
                    }
                    BeamDirection::Right => {
                        self.direction = BeamDirection::Up;
                    }
                }
                self.position = self.direction.next_natural_position(&self.position);
            }
            '\\' => {
                match self.direction {
                    BeamDirection::Up => {
                        self.direction = BeamDirection::Left;
                    }
                    BeamDirection::Down => {
                        self.direction = BeamDirection::Right;
                    }
                    BeamDirection::Left => {
                        self.direction = BeamDirection::Up;
                    }
                    BeamDirection::Right => {
                        self.direction = BeamDirection::Down;
                    }
                }
                self.position = self.direction.next_natural_position(&self.position);
            }
            '.' => {
                self.position = self.direction.next_natural_position(&self.position);
            }
            _ => {
                panic!("Invalid tile: {}", tile);
            }
        }
        new_beam
    }
}

fn traverse(
    input: &Vec<Vec<char>>,
    initial_beam: Beam,
    energized_tiles: &mut HashSet<(usize, usize)>,
    existing_beams: &mut HashSet<Beam>,
) {
    let width = input[0].len();
    let height = input.len();
    let mut beam = initial_beam;
    // println!("Traverse Begin {:?}", beam);

    while beam.position.is_valid(width, height) && !existing_beams.contains(&beam) {
        existing_beams.insert(beam);
        let tile = input[beam.position.y][beam.position.x];
        // println!("{},{}", beam.position.x, beam.position.y);
        energized_tiles.insert((beam.position.y, beam.position.x));
        if let Some(new_beam) = beam.move_forward(tile) {
            traverse(input, new_beam, energized_tiles, existing_beams);
        }
    }
}

pub fn part1() {
    let input: Vec<Vec<char>> = utils::read_lines("./inputs/day16")
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let initial_beam = Beam::new(BeamDirection::Right, Position { x: 0, y: 0 });
    let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
    let mut existing_beams: HashSet<Beam> = HashSet::new();
    traverse(
        &input,
        initial_beam,
        &mut energized_tiles,
        &mut existing_beams,
    );
    println!("{}", energized_tiles.len());
}

pub fn part2() {
    let input: Vec<Vec<char>> = utils::read_lines("./inputs/day16")
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    let mut initial_beams: Vec<Beam> = vec![];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if y == 0 {
                initial_beams.push(Beam::new(BeamDirection::Down, Position { x, y }));
            }
            if x == 0 {
                initial_beams.push(Beam::new(BeamDirection::Right, Position { x, y }));
            }
            if y == input.len() - 1 {
                initial_beams.push(Beam::new(BeamDirection::Up, Position { x, y }));
            }
            if x == input[0].len() - 1 {
                initial_beams.push(Beam::new(BeamDirection::Left, Position { x, y }));
            }
        }
    }

    let mut max_energized_tiles = 0;
    for initial_beam in initial_beams {
        let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
        let mut existing_beams: HashSet<Beam> = HashSet::new();
        traverse(
            &input,
            initial_beam,
            &mut energized_tiles,
            &mut existing_beams,
        );
        if energized_tiles.len() > max_energized_tiles {
            max_energized_tiles = energized_tiles.len();
        }
    }
    println!("{}", max_energized_tiles);
}
