use std::collections::HashMap;

use crate::utils;

#[derive(Debug)]
struct PartStats {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl PartStats {
    pub fn new(input: String) -> Self {
        let mut part_stats = PartStats {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for stat in input.split(',') {
            let mut parts = stat.split('=');
            match parts.next().unwrap() {
                "x" => part_stats.x = parts.collect::<String>().parse::<u64>().unwrap(),
                "m" => part_stats.m = parts.collect::<String>().parse::<u64>().unwrap(),
                "a" => part_stats.a = parts.collect::<String>().parse::<u64>().unwrap(),
                "s" => part_stats.s = parts.collect::<String>().parse::<u64>().unwrap(),
                _ => {}
            }
        }
        part_stats
    }
}

#[derive(Debug)]
enum ConditionResultStatus {
    Redirected,
    Accepted,
    Rejected,
}

#[derive(Debug)]
struct ConditionResult {
    status: ConditionResultStatus,
    value: String,
}

#[derive(Debug)]
struct Condition {
    stat: char,
    comparator: char,
    value: u64,
    result: ConditionResult,
}

impl Condition {
    pub fn new(input: &str) -> Self {
        let mut condition_parts = input.split(':');
        let (stat, comparator, value) = if condition_parts.clone().count() > 1 {
            let mut condition_iter = condition_parts.next().unwrap().chars();
            (
                condition_iter.next().unwrap(),
                condition_iter.next().unwrap(),
                condition_iter.collect::<String>().parse::<u64>().unwrap(),
            )
        } else {
            (' ', ' ', 0)
        };
        let result = match condition_parts.next().unwrap() {
            "A" => ConditionResult {
                status: ConditionResultStatus::Accepted,
                value: "".to_string(),
            },
            "R" => ConditionResult {
                status: ConditionResultStatus::Rejected,
                value: "".to_string(),
            },
            _workflow => ConditionResult {
                status: ConditionResultStatus::Redirected,
                value: _workflow.to_string(),
            },
        };
        Condition {
            stat,
            comparator,
            value,
            result,
        }
    }
}

#[derive(Debug)]
struct WorkFlow(Vec<Condition>);

impl WorkFlow {
    pub fn new(input: String) -> Self {
        let mut conditions: Vec<Condition> = vec![];
        for condition in input.split(',') {
            conditions.push(Condition::new(condition));
        }
        WorkFlow(conditions)
    }
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day19_sample");
    let mut workflows: HashMap<String, WorkFlow> = HashMap::new();
    let mut parsing_workflows = true;
    let mut parts: Vec<PartStats> = vec![];

    for line in lines {
        if line.is_empty() {
            parsing_workflows = false;
            continue;
        }
        if parsing_workflows {
            let mut parts = line.split('{');
            let workflow_name = parts.next().unwrap().to_string();
            let workflow = WorkFlow::new(parts.next().unwrap().replace('}', ""));
            workflows.insert(workflow_name, workflow);
        } else {
            parts.push(PartStats::new(line.replace(['{', '}'], "")));
        }
    }
    for workflow in workflows {
        println!("{:?}", workflow);
    }
    for part in parts {
        println!("{:?}", part);
    }
}

pub fn part2() {}
