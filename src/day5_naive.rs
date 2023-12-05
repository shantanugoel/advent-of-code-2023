use crate::utils;
use regex::Regex;
use std::collections::HashMap;

fn process_map_line(line: &String, map: &mut HashMap<u64, u64>) {
    let re = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
    let cap = re.captures(line).unwrap();
    let destination = cap[1].parse::<u64>().unwrap();
    let source = cap[2].parse::<u64>().unwrap();
    let range = cap[3].parse::<u64>().unwrap();
    for i in 0..range {
        map.insert(source + i, destination + i);
    }
}

fn process_seeds(line: &str, seeds: &mut Vec<u64>) {
    let re = Regex::new(r"(\d+)").unwrap();
    seeds.extend(
        re.find_iter(line)
            .map(|m| m.as_str().parse::<u64>().unwrap()),
    );
}

pub fn part1() {
    let mut seed_soil: HashMap<u64, u64> = HashMap::new();
    let mut soil_fertilizer: HashMap<u64, u64> = HashMap::new();
    let mut fertilizer_water: HashMap<u64, u64> = HashMap::new();
    let mut water_light: HashMap<u64, u64> = HashMap::new();
    let mut light_temp: HashMap<u64, u64> = HashMap::new();
    let mut temp_humidity: HashMap<u64, u64> = HashMap::new();
    let mut humidity_loc: HashMap<u64, u64> = HashMap::new();

    let lines = utils::read_lines("./inputs/day5");
    let mut current_map: &mut HashMap<u64, u64> = &mut seed_soil;
    let mut seeds: Vec<u64> = vec![];
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
                    process_map_line(&line, &mut current_map);
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
    for map in map_chain.iter() {}
    let mut locations: Vec<u64> = vec![];
    for seed in seeds {
        println!("=====");
        let mut key = seed;
        for map in map_chain.iter() {
            key = *map.get(&key).unwrap_or(&key);
        }
        locations.push(key);
    }
    println!("{}", locations.iter().min().unwrap());
}
