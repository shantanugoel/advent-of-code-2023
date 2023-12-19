use std::collections::HashMap;

use crate::utils;

#[derive(Debug, Clone, Copy)]
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

    pub fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConditionResultStatus {
    Redirected,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone)]
struct ConditionResult {
    status: ConditionResultStatus,
    value: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

fn parse_workflow(part: PartStats, workflows: &HashMap<String, WorkFlow>) -> ConditionResultStatus {
    let mut result: ConditionResultStatus = ConditionResultStatus::Redirected;
    let mut workflow_key = "in";
    let mut workflow: &WorkFlow;
    loop {
        match result {
            ConditionResultStatus::Accepted => break,
            ConditionResultStatus::Rejected => break,
            ConditionResultStatus::Redirected => workflow = workflows.get(workflow_key).unwrap(),
        }
        for rule in &workflow.0 {
            let part_value = match rule.stat {
                'x' => Some(part.x),
                'm' => Some(part.m),
                'a' => Some(part.a),
                's' => Some(part.s),
                _ => None,
            };
            let condition_satisfied = match rule.comparator {
                '>' => part_value.unwrap() > rule.value,
                '<' => part_value.unwrap() < rule.value,
                _ => true,
            };
            if condition_satisfied {
                result = rule.result.status.clone();
                workflow_key = rule.result.value.as_str();
                break;
            }
        }
    }
    result
}

pub fn part1() {
    let lines = utils::read_lines("./inputs/day19");
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
    // for workflow in workflows {
    //     println!("{:?}", workflow);
    // }
    let mut sum = 0;
    for part in parts {
        if parse_workflow(part, &workflows) == ConditionResultStatus::Accepted {
            sum += part.sum();
        }
    }
    println!("{}", sum);
}

pub fn part2() {}
