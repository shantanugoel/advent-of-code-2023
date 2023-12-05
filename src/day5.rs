use crate::utils;
use crossbeam::thread;
use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug)]
struct MapData {
    destination: i64,
    range: i64,
}

fn process_map_line(line: &str, map: &mut BTreeMap<i64, MapData>) {
    let re = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
    let cap = re.captures(line).unwrap();
    let destination = cap[1].parse::<i64>().unwrap();
    let source = cap[2].parse::<i64>().unwrap();
    let range = cap[3].parse::<i64>().unwrap();
    map.insert(source, MapData { destination, range });
}

fn process_seeds(line: &str, seeds: &mut Vec<i64>) {
    let re = Regex::new(r"(\d+)").unwrap();
    seeds.extend(
        re.find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap()),
    );
}

fn find_neighboring_keys(map: &BTreeMap<i64, MapData>, key: i64) -> Option<(i64, i64)> {
    let mut iter = map.range(..=key);
    if let Some((&lower_key, _)) = iter.next_back() {
        let upper_key = iter.next().map(|(&k, _)| k).unwrap_or(lower_key);
        Some((lower_key, upper_key))
    } else {
        None
    }
}

pub fn part1() {
    let mut seed_soil: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut soil_fertilizer: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut fertilizer_water: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut water_light: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut light_temp: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut temp_humidity: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut humidity_loc: BTreeMap<i64, MapData> = BTreeMap::new();

    let lines = utils::read_lines("./inputs/day5");
    let mut current_map: &mut BTreeMap<i64, MapData> = &mut seed_soil;
    let mut seeds: Vec<i64> = vec![];
    for line in lines {
        match line.split_once(':') {
            Some((prefix, suffix)) => match prefix {
                "seeds" => {
                    process_seeds(suffix, &mut seeds);
                }
                "seed-to-soil map" => {
                    current_map = &mut seed_soil;
                }
                "soil-to-fertilizer map" => {
                    current_map = &mut soil_fertilizer;
                }
                "fertilizer-to-water map" => {
                    current_map = &mut fertilizer_water;
                }
                "water-to-light map" => {
                    current_map = &mut water_light;
                }
                "light-to-temperature map" => {
                    current_map = &mut light_temp;
                }
                "temperature-to-humidity map" => {
                    current_map = &mut temp_humidity;
                }
                "humidity-to-location map" => {
                    current_map = &mut humidity_loc;
                }
                _ => {}
            },
            None => {
                if !line.is_empty() {
                    process_map_line(&line.as_str(), &mut current_map);
                }
            }
        }
    }

    let map_chain = [
        &seed_soil,
        &soil_fertilizer,
        &fertilizer_water,
        &water_light,
        &light_temp,
        &temp_humidity,
        &humidity_loc,
    ];
    let mut locations: Vec<i64> = vec![];
    for seed in seeds {
        let mut key = seed;
        for map in map_chain.iter() {
            let temp = find_neighboring_keys(map, key);
            if let Some((lower_key, _)) = temp {
                let val = map.get(&lower_key).unwrap();
                if key <= lower_key + val.range {
                    key += val.destination - lower_key;
                }
            }
        }
        locations.push(key);
    }
    println!("{}", locations.iter().min().unwrap());
}

pub fn part2() {
    let mut seed_soil: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut soil_fertilizer: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut fertilizer_water: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut water_light: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut light_temp: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut temp_humidity: BTreeMap<i64, MapData> = BTreeMap::new();
    let mut humidity_loc: BTreeMap<i64, MapData> = BTreeMap::new();

    let lines = utils::read_lines("./inputs/day5");
    let mut current_map: &mut BTreeMap<i64, MapData> = &mut seed_soil;
    let mut seeds: Vec<i64> = vec![];
    for line in lines {
        match line.split_once(':') {
            Some((prefix, suffix)) => match prefix {
                "seeds" => {
                    process_seeds(suffix, &mut seeds);
                }
                "seed-to-soil map" => {
                    current_map = &mut seed_soil;
                }
                "soil-to-fertilizer map" => {
                    current_map = &mut soil_fertilizer;
                }
                "fertilizer-to-water map" => {
                    current_map = &mut fertilizer_water;
                }
                "water-to-light map" => {
                    current_map = &mut water_light;
                }
                "light-to-temperature map" => {
                    current_map = &mut light_temp;
                }
                "temperature-to-humidity map" => {
                    current_map = &mut temp_humidity;
                }
                "humidity-to-location map" => {
                    current_map = &mut humidity_loc;
                }
                _ => {}
            },
            None => {
                if !line.is_empty() {
                    process_map_line(&line.as_str(), &mut current_map);
                }
            }
        }
    }

    let map_chain = [
        &seed_soil,
        &soil_fertilizer,
        &fertilizer_water,
        &water_light,
        &light_temp,
        &temp_humidity,
        &humidity_loc,
    ];
    let mut locations: Vec<i64> = vec![];
    let process_chunk = |chunk: &[i64]| {
        let mut min = i64::MAX;
        for i in 0..chunk[1] {
            let mut key = chunk[0] + i;

            for map in map_chain.iter() {
                let temp = find_neighboring_keys(map, key);
                if let Some((lower_key, _)) = temp {
                    let val = map.get(&lower_key).unwrap();
                    if key <= lower_key + val.range {
                        key += val.destination - lower_key;
                    }
                }
            }

            if key < min {
                min = key;
            }
        }
        min
    };

    let chunks = seeds.chunks_exact(2);

    thread::scope(|s| {
        let handles: Vec<_> = chunks
            .map(|chunk| s.spawn(move |_| process_chunk(chunk)))
            .collect();

        for handle in handles {
            locations.push(handle.join().unwrap());
        }
    })
    .unwrap();
    println!("{}", locations.iter().min().unwrap());
}
