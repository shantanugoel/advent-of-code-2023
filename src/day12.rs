use std::collections::HashMap;
use std::sync::Mutex;

use crate::utils;
use crossbeam::thread;
use lazy_static::lazy_static;

lazy_static! {
    static ref CACHE: Mutex<HashMap<(String, Vec<i64>, State, i64), u64>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum State {
    Damaged,
    Undamaged,
}

fn parse_record(
    record_1: &str,
    record_2: &[i64],
    beginning_state: State,
    beginning_count: i64,
    sum: &mut u64,
) {
    let mut state = beginning_state;
    let mut record_1_iter = record_1.chars().peekable();
    let mut record_2_iter = record_2.iter().copied().peekable();
    let mut current_count_in_record_2 = beginning_count;
    let mut not_found = false;
    let (a, b, c, d) = (
        record_1.to_string().clone(),
        record_2.to_vec().clone(),
        state,                     // state,
        current_count_in_record_2, // current_count_in_record_2,
    );
    let sum3 = *sum;
    let cache = CACHE.lock().unwrap();
    if cache.contains_key(&(a.clone(), b.clone(), c, d)) {
        *sum += cache.get(&(a.clone(), b.clone(), c, d)).unwrap();
        return;
    }
    drop(cache);
    loop {
        match record_1_iter.next() {
            Some('#') => {
                if current_count_in_record_2 == 0 {
                    if state == State::Damaged {
                        not_found = true;
                        break;
                    }
                    if let Some(current_count) = record_2_iter.next() {
                        current_count_in_record_2 = current_count;
                    } else {
                        not_found = true;
                        break;
                    }
                }
                current_count_in_record_2 -= 1;
                if current_count_in_record_2 < 0 {
                    not_found = true;
                    break;
                }
                state = State::Damaged;
                continue;
            }
            Some('.') => {
                if state == State::Damaged && current_count_in_record_2 != 0 {
                    not_found = true;
                    break;
                }
                state = State::Undamaged;
                continue;
            }

            Some('?') => {
                let mut temp_record_1 = std::iter::once('#')
                    .chain(record_1_iter.clone())
                    .collect::<String>();
                let temp_record_2 = record_2_iter.clone().collect::<Vec<i64>>();
                // Can be either . or # at this point.
                if current_count_in_record_2 != 0 || state != State::Damaged {
                    parse_record(
                        temp_record_1.as_str(),
                        &temp_record_2,
                        state,
                        current_count_in_record_2,
                        sum,
                    );
                }
                if current_count_in_record_2 == 0 && record_1_iter.peek().is_none() {
                    break;
                }
                temp_record_1 = std::iter::once('.')
                    .chain(record_1_iter.clone())
                    .collect::<String>();
                if current_count_in_record_2 == 0 || state == State::Undamaged {
                    parse_record(
                        temp_record_1.as_str(),
                        &temp_record_2,
                        state,
                        current_count_in_record_2,
                        sum,
                    );
                }
                break;
            }
            _ => break,
        }
    }
    if record_1_iter.next().is_none()
        && record_2_iter.next().is_none()
        && current_count_in_record_2 == 0
        && !not_found
    {
        *sum += 1;
    }
    let mut cache = CACHE.lock().unwrap();
    // println!(
    //     "cache put default  {} {:?} {:?} {} --> {}",
    //     a.clone(),
    //     b.clone(),
    //     c,
    //     d,
    //     *sum - sum3
    // );
    cache
        .entry((a.clone(), b.clone(), c, d))
        .or_insert(*sum - sum3);
    drop(cache);
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day12_sample");

    let records: Vec<(String, Vec<i64>)> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let record_1 = parts.next().unwrap().to_string();
            let record_2 = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (record_1, record_2)
        })
        .collect();
    let mut sum = 0;
    records.iter().for_each(|(record_1, record_2)| {
        parse_record(record_1, record_2, State::Undamaged, 0, &mut sum)
    });

    println!("{} ", sum);
}

fn expand(records: &Vec<(String, Vec<i64>)>) -> (Vec<String>, Vec<Vec<i64>>) {
    let mut r1: Vec<String> = vec![];
    let mut r2: Vec<Vec<i64>> = vec![];
    for record in records {
        let temp_r1: String = std::iter::repeat(record.0.clone())
            .take(5)
            .collect::<Vec<_>>()
            .join("?");
        let temp_r2: Vec<i64> = record
            .1
            .iter()
            .cloned()
            .cycle()
            .take(record.1.len() * 5)
            .collect();
        r1.push(temp_r1);
        r2.push(temp_r2);
    }
    (r1, r2)
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day12");

    let records: Vec<(String, Vec<i64>)> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let record_1 = parts.next().unwrap().to_string();
            let record_2 = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (record_1, record_2)
        })
        .collect();

    let (r1, r2) = expand(&records);
    let mut sum = 0;

    for i in 0..records.len() {
        parse_record(&r1[i], &r2[i], State::Undamaged, 0, &mut sum);
        println!("{}", i);
    }

    println!("{} ", sum);
}
