use crate::utils;

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Damaged,
    Undamaged,
}

fn parse_record(
    record_1: &str,
    record_2: &[i32],
    beginning_state: State,
    beginning_count: i32,
    sum: &mut u32,
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
        //     record_2_iter.clone().collect::<Vec<i32>>(),
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
                // Can be either . or # at this point.
                if current_count_in_record_2 != 0 || state != State::Damaged {
                    parse_record(
                        std::iter::once('#')
                            .chain(record_1_iter.clone())
                            .collect::<String>()
                            .as_str(),
                        &record_2_iter.clone().collect::<Vec<i32>>(),
                        state,
                        current_count_in_record_2,
                        sum,
                    );
                }
                println!("===== {}", sum);
                if current_count_in_record_2 == 0 && record_1_iter.peek().is_none() {
                    break;
                }
                if current_count_in_record_2 == 0 || state == State::Undamaged {
                    parse_record(
                        std::iter::once('.')
                            .chain(record_1_iter.clone())
                            .collect::<String>()
                            .as_str(),
                        &record_2_iter.clone().collect::<Vec<i32>>(),
                        state,
                        current_count_in_record_2,
                        sum,
                    );
                }
                println!("****** {}", sum);
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
        println!("Found a solution: {}", *sum + 1);
        *sum += 1;
    }
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day12");

    let records: Vec<(String, Vec<i32>)> = lines
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

pub fn part2() {}
