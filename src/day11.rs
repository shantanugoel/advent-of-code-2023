use crate::utils;
use crossbeam::thread;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn expand(map: &Vec<Vec<char>>, expansion_factor: u32) -> Vec<Vec<u32>> {
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
            for _ in 0..expansion_factor {
                expanded_map.push(new_row.clone());
            }
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
            for _ in 0..expansion_factor {
                offset += 1;
                for row in expanded_map.iter_mut() {
                    row.insert(col_index + offset, 0);
                }
            }
        }
    }
    expanded_map
}

fn expand2(
    map: &Vec<Vec<char>>,
    expansion_factor: u32,
    rows: &mut Vec<usize>,
    cols: &mut Vec<usize>,
) -> Vec<Vec<u32>> {
    let mut expanded_map: Vec<Vec<u32>> = vec![];
    let mut galaxy_count = 0;
    let mut count = 0;
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
        count += 1;
        if empty_row {
            rows.push(count);
            for _ in 0..expansion_factor {
                count += 1;
                expanded_map.push(new_row.clone());
            }
        }
    }

    let mut offset = 0;
    count = 0;
    for col_index in 0..map[0].len() {
        count += 1;
        let mut empty_column = true;
        for row in expanded_map.iter() {
            if row[col_index + offset] != 0 {
                empty_column = false;
            }
        }
        if empty_column {
            cols.push(count);
            for _ in 0..expansion_factor {
                count += 1;
                offset += 1;
                for row in expanded_map.iter_mut() {
                    row.insert(col_index + offset, 0);
                }
            }
        }
    }
    expanded_map
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point(u32, u32);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    point: Point,
    steps: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(point: Point, rows: usize, cols: usize) -> Vec<Point> {
    let mut result = Vec::new();

    if point.0 > 0 {
        result.push(Point(point.0 - 1, point.1));
    }
    if point.1 > 0 {
        result.push(Point(point.0, point.1 - 1));
    }
    if point.0 + 1 < rows as u32 {
        result.push(Point(point.0 + 1, point.1));
    }
    if point.1 + 1 < cols as u32 {
        result.push(Point(point.0, point.1 + 1));
    }

    result
}

fn dijkstra(matrix: &Vec<Vec<u32>>, start: Point, end: Point) -> Option<(u32, Point, Point)> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut distances: HashMap<Point, u32> = HashMap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start, 0);
    heap.push(State {
        point: start,
        steps: 0,
    });

    while let Some(State { point, steps }) = heap.pop() {
        if point == end {
            return Some((steps, start, end));
        }

        if visited.contains(&point) {
            continue;
        }

        visited.insert(point);

        for neighbor in neighbors(point, rows, cols) {
            let new_steps = steps + 1;
            if !distances.contains_key(&neighbor) || new_steps < *distances.get(&neighbor).unwrap()
            {
                distances.insert(neighbor, new_steps);
                heap.push(State {
                    point: neighbor,
                    steps: new_steps,
                });
            }
        }
    }

    None
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day11_sample");
    let map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let expanded_map = expand(&map, 1);
    let mut galaxies: Vec<Point> = vec![];
    for (row_index, row) in expanded_map.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col != 0 {
                galaxies.push(Point(row_index as u32, col_index as u32));
            }
        }
    }
    let mut sum = 0;
    let mut count = 0;
    for (index, origin) in galaxies.iter().enumerate() {
        for destination in galaxies.iter().skip(index) {
            if *origin == *destination {
                continue;
            }
            if let Some((distance, _, _)) = dijkstra(&expanded_map, *origin, *destination) {
                count += 1;
                sum += distance;
            }
        }
    }

    println!("{} {}", count, sum);
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day11");
    let map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut rows: Vec<usize> = vec![];
    let mut cols: Vec<usize> = vec![];
    let expanded_map = expand2(&map, 10 - 1, &mut rows, &mut cols);
    let mut galaxies: Vec<Point> = vec![];
    for (row_index, row) in expanded_map.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col != 0 {
                galaxies.push(Point(row_index as u32, col_index as u32));
            }
        }
    }
    let mut sum = 0;
    let mut count = 0;
    let mut permutations: Vec<(Point, Point)> = vec![];
    for (index, origin) in galaxies.iter().enumerate() {
        for destination in galaxies.iter().skip(index) {
            if *origin == *destination {
                continue;
            }
            permutations.push((*origin, *destination));
        }
    }

    thread::scope(|s| {
        let handles: Vec<_> = permutations
            .iter()
            .map(|chunk| s.spawn(|_| dijkstra(&expanded_map, chunk.0, chunk.1)))
            .collect();
        for handle in handles {
            if let Some((distance, start, end)) = handle.join().unwrap() {
                count += 1;
                let mut expanded_rows = 0;
                let mut expanded_cols = 0;
                for y in rows.iter() {
                    if (*y >= start.0 as usize && *y <= end.0 as usize)
                        || (*y <= start.0 as usize && *y >= end.0 as usize)
                    {
                        expanded_rows += 1;
                    }
                }

                for x in cols.iter() {
                    if (*x >= start.1 as usize && *x <= end.1 as usize)
                        || (*x <= start.1 as usize && *x >= end.1 as usize)
                    {
                        expanded_cols += 1;
                    }
                }
                let mut row_factor: u64 = 0;
                let mut col_factor: u64 = 0;
                for _ in 0..expanded_rows {
                    row_factor += 100_000 - 1;
                }
                for _ in 0..expanded_cols {
                    col_factor += 100_000 - 1;
                }

                let distance2: u64 = distance as u64 + (row_factor + col_factor) * 10;
                sum += distance2;
            }
        }
    })
    .unwrap();

    println!("{} {}", count, sum);
}
