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
    let mut sum = 0;
    for part in parts {
        if parse_workflow(part, &workflows) == ConditionResultStatus::Accepted {
            sum += part.sum();
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    let lines = utils::read_lines("./inputs/day19");
    let mut workflows: HashMap<String, WorkFlow> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('{');
        let workflow_name = parts.next().unwrap().to_string();
        let workflow = WorkFlow::new(parts.next().unwrap().replace('}', ""));
        workflows.insert(workflow_name, workflow);
    }

    let mut total: u64 = 0;
    find_combinations(
        &workflows,
        "in".to_string(),
        0,
        Combination {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        &mut total,
    );
    println!("{}", total);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Combination {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl Combination {
    pub fn value(&self) -> u64 {
        let x = self.x.1 - self.x.0 + 1;
        let m = self.m.1 - self.m.0 + 1;
        let a = self.a.1 - self.a.0 + 1;
        let s = self.s.1 - self.s.0 + 1;
        println!("Value: {:?}", self);
        x * m * a * s
    }
}

pub fn update_combination(
    old_min_max: (u64, u64),
    rule_value: u64,
    comparator: char,
    positive_path: bool,
) -> (u64, u64) {
    if positive_path {
        match comparator {
            '<' => (old_min_max.0, old_min_max.1.min(rule_value - 1)),
            '>' => (old_min_max.0.max(rule_value + 1), old_min_max.1),
            _ => old_min_max,
        }
    } else {
        match comparator {
            '<' => (old_min_max.0.max(rule_value), old_min_max.1),
            '>' => (old_min_max.0, old_min_max.1.min(rule_value)),
            _ => old_min_max,
        }
    }
}

fn find_combinations(
    workflows: &HashMap<String, WorkFlow>,
    workflow_key: String,
    rule_index: usize,
    combinations: Combination,
    total: &mut u64,
) {
    let mut current_combinations_positive = combinations;
    let mut current_combinations_negative = combinations;

    let workflow = workflows.get(&workflow_key).unwrap();
    if let Some(rule) = workflow.0.get(rule_index) {
        println!("{}, combo: {:?}", workflow_key, combinations);
        match rule.stat {
            'x' => {
                current_combinations_positive.x = update_combination(
                    current_combinations_positive.x,
                    rule.value,
                    rule.comparator,
                    true,
                );
                current_combinations_negative.x = update_combination(
                    current_combinations_negative.x,
                    rule.value,
                    rule.comparator,
                    false,
                );
            }
            'm' => {
                current_combinations_positive.m = update_combination(
                    current_combinations_positive.m,
                    rule.value,
                    rule.comparator,
                    true,
                );
                current_combinations_negative.m = update_combination(
                    current_combinations_negative.m,
                    rule.value,
                    rule.comparator,
                    false,
                );
            }
            'a' => {
                current_combinations_positive.a = update_combination(
                    current_combinations_positive.a,
                    rule.value,
                    rule.comparator,
                    true,
                );
                current_combinations_negative.a = update_combination(
                    current_combinations_negative.a,
                    rule.value,
                    rule.comparator,
                    false,
                );
            }
            's' => {
                current_combinations_positive.s = update_combination(
                    current_combinations_positive.s,
                    rule.value,
                    rule.comparator,
                    true,
                );
                current_combinations_negative.s = update_combination(
                    current_combinations_negative.s,
                    rule.value,
                    rule.comparator,
                    false,
                );
            }
            _ => {}
        }
        match rule.result.status {
            ConditionResultStatus::Accepted => {
                *total += current_combinations_positive.value();
                println!(
                    "----------Accepted => {}: {}, {} - {:?}\n{:?}",
                    workflow_key,
                    current_combinations_positive.value(),
                    total,
                    rule,
                    current_combinations_positive
                );
                find_combinations(
                    workflows,
                    workflow_key,
                    rule_index + 1,
                    current_combinations_negative,
                    total,
                );
                // }
            }
            ConditionResultStatus::Rejected => {
                println!("Rejected => {:?}", rule);
                find_combinations(
                    workflows,
                    workflow_key,
                    rule_index + 1,
                    current_combinations_negative,
                    total,
                );
            }
            ConditionResultStatus::Redirected => {
                // println!("Redirected => {:?}", rule);
                find_combinations(
                    workflows,
                    rule.result.value.clone(),
                    0,
                    current_combinations_positive,
                    total,
                );
                find_combinations(
                    workflows,
                    workflow_key,
                    rule_index + 1,
                    current_combinations_negative,
                    total,
                );
            }
        }
    }
}
