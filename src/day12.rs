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

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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
    // println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    loop {
        // println!(
        //     "Parsing record 1: {}, record 2 {:?} current count {}",
        //     record_1_iter.clone().collect::<String>(),
        //     record_2_iter.clone().collect::<Vec<i64>>(),
        //     current_count_in_record_2
        // );
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
                        // println!("99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999");
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
                let temp_record_1_orig = std::iter::once('?')
                    .chain(record_1_iter.clone())
                    .collect::<String>();
                let temp_record_2 = record_2_iter.clone().collect::<Vec<i64>>();
                let cache = CACHE.lock().unwrap();
                if cache.contains_key(&(
                    temp_record_1_orig.clone(),
                    temp_record_2.clone(),
                    state,
                    current_count_in_record_2,
                )) {
                    // println!("visited");
                    // println!("{}", sum);
                    *sum += cache
                        .get(&(
                            temp_record_1_orig.clone(),
                            temp_record_2.clone(),
                            state,
                            current_count_in_record_2,
                        ))
                        .unwrap();
                    // println!("cache get {} {:?}", temp_record_1_orig, temp_record_2);
                    // println!("{}", sum);
                    return;
                }
                drop(cache);
                // Can be either . or # at this point.
                let mut sum2: u64 = *sum;
                if current_count_in_record_2 != 0 || state != State::Damaged {
                    parse_record(
                        // std::iter::once('#')
                        //     .chain(record_1_iter.clone())
                        //     .collect::<String>()
                        //     .as_str(),
                        temp_record_1.as_str(),
                        &temp_record_2,
                        state,
                        current_count_in_record_2,
                        sum,
                    );
                }
                if sum2 < *sum {
                    let mut cache = CACHE.lock().unwrap();
                    // println!(
                    //     "Found a solution: {}, Cache Length {}",
                    //     *sum - sum2,
                    //     cache.len()
                    // );
                    // println!(
                    //     "cache put # {} {:?}",
                    //     temp_record_1_orig.clone(),
                    //     temp_record_2.clone()
                    // );
                    // if cache.len() > 10 {
                    //     panic!("test");
                    // }
                    cache.insert(
                        (
                            temp_record_1_orig.clone(),
                            temp_record_2.clone(),
                            state,
                            current_count_in_record_2,
                        ),
                        *sum - sum2,
                    );
                    drop(cache);
                }
                // println!("===== {}", sum);
                sum2 = *sum;
                if current_count_in_record_2 == 0 && record_1_iter.peek().is_none() {
                    break;
                }
                temp_record_1 = std::iter::once('.')
                    .chain(record_1_iter.clone())
                    .collect::<String>();
                if current_count_in_record_2 == 0 || state == State::Undamaged {
                    parse_record(
                        // std::iter::once('.')
                        //     .chain(record_1_iter.clone())
                        //     .collect::<String>()
                        //     .as_str(),
                        temp_record_1.as_str(),
                        &temp_record_2,
                        state,
                        current_count_in_record_2,
                        sum,
                    );
                }
                if sum2 < *sum {
                    let mut cache = CACHE.lock().unwrap();
                    // println!(
                    //     "Found a solution: {}, Cache Length {}",
                    //     *sum - sum2,
                    //     cache.len()
                    // );
                    // println!(
                    //     "cache put . {} {:?}",
                    //     temp_record_1_orig.clone(),
                    //     temp_record_2.clone()
                    // );
                    // if cache.len() > 10 {
                    //     panic!("test");
                    // }
                    // cache.insert(
                    //     (
                    //         temp_record_1.clone(),
                    //         temp_record_2.clone(),
                    //         state,
                    //         current_count_in_record_2,
                    //     ),
                    //     *sum - sum2,
                    // );
                    cache
                        .entry((
                            temp_record_1_orig.clone(),
                            temp_record_2.clone(),
                            state,
                            current_count_in_record_2,
                        ))
                        .and_modify(|v| *v += *sum - sum2)
                        .or_insert(*sum - sum2);
                    drop(cache);
                }
                // println!("****** {}", sum);
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

    // let mut x = 0;
    // for record in records {
    //     parse_record(
    //         record.0.as_str(),
    //         record.1.as_slice(),
    //         State::Undamaged,
    //         &mut x,
    //     );
    // }
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

    // println!("{}", r1[0]);
    // println!("{:?}", r2[0]);
    // for i in 0..records.len() {
    //     parse_record(&r1[i], &r2[i], State::Undamaged, 0, &mut sum);
    // }

    struct T {
        s1: String,
        s2: Vec<i64>,
    }

    let mut r: Vec<T> = vec![];

    for i in 0..records.len() {
        r.push(T {
            s1: r1[i].clone(),
            s2: r2[i].clone(),
        });
    }

    let result = thread::scope(|s| {
        let handles: Vec<_> = r
            .iter()
            .map(|r| {
                s.spawn(|_| {
                    let mut s = 0;
                    parse_record(&r.s1, &r.s2, State::Undamaged, 0, &mut s);
                    unsafe {
                        static mut p: u32 = 0;
                        p += 1;
                        println!("{}", p);
                    }
                    s
                })
            })
            .collect();
        for handle in handles {
            sum += handle.join().unwrap();
            println!("sum: {}", sum);
        }
        sum
    })
    .unwrap();

    // records.iter().for_each(|(record_1, record_2)| {
    //     parse_record(record_1, record_2, State::Undamaged, 0, &mut sum)
    // });

    // let mut x = 0;
    // for record in records {
    //     parse_record(
    //         record.0.as_str(),
    //         record.1.as_slice(),
    //         State::Undamaged,
    //         &mut x,
    //     );
    // }
    println!("{} ", result);
}
