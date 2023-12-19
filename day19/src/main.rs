use std::collections::HashMap;

use regex::Regex;

type RuleCondition = Box<dyn Fn(u32) -> bool>;

enum Rule {
    X(RuleCondition),
    M(RuleCondition),
    A(RuleCondition),
    S(RuleCondition),
    Pass,
}

struct Workflow {
    rules: Vec<(Rule, String)>,
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (workflows_str, part_ratings) = input.split_once("\n\n").unwrap();
    let mut list: HashMap<String, Workflow> = HashMap::new();

    for workflow in workflows_str.lines() {
        let mut parts = workflow.split(&['{', '}']);
        let name = parts.next().unwrap();
        let rules = parts.next().unwrap();

        let mut rules_v = Vec::new();
        for rule in rules.split(',') {
            if let Some((condition, next_workflow)) = rule.split_once(':') {
                if let Some((category, value)) = condition.split_once('<') {
                    let value = value.parse::<u32>().unwrap();
                    let rule = match category {
                        "x" => Rule::X(Box::new(move |v| v < value)),
                        "m" => Rule::M(Box::new(move |v| v < value)),
                        "a" => Rule::A(Box::new(move |v| v < value)),
                        "s" => Rule::S(Box::new(move |v| v < value)),
                        _ => panic!("invalid input"),
                    };
                    rules_v.push((rule, next_workflow.to_string()));
                } else if let Some((category, value)) = condition.split_once('>') {
                    let value = value.parse::<u32>().unwrap();
                    let rule = match category {
                        "x" => Rule::X(Box::new(move |v| v > value)),
                        "m" => Rule::M(Box::new(move |v| v > value)),
                        "a" => Rule::A(Box::new(move |v| v > value)),
                        "s" => Rule::S(Box::new(move |v| v > value)),
                        _ => panic!("invalid input"),
                    };
                    rules_v.push((rule, next_workflow.to_string()));
                }
            } else {
                rules_v.push((Rule::Pass, rule.to_string()));
            }
        }
        list.insert(name.to_string(), Workflow { rules: rules_v });
    }

    let mut parts = vec![];
    let re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
    for rating in part_ratings.lines() {
        if let Some(caps) = re.captures(rating) {
            parts.push(Part {
                x: caps["x"].parse().unwrap(),
                m: caps["m"].parse().unwrap(),
                a: caps["a"].parse().unwrap(),
                s: caps["s"].parse().unwrap(),
            })
        }
    }

    let mut sum = 0;

    for part in parts {
        let mut current_rule = "in".to_owned();
        'outer: loop {
            if current_rule == "A" {
                sum += part.x + part.m + part.a + part.s;
                break;
            } else if current_rule == "R" {
                break;
            }
            let workflow = list.get(&current_rule).unwrap();
            'inner: for (rule, next) in workflow.rules.iter() {
                match rule {
                    Rule::X(cond) => {
                        if cond(part.x) {
                            current_rule = next.to_owned();
                            break 'inner;
                        }
                    }
                    Rule::M(cond) => {
                        if cond(part.m) {
                            current_rule = next.to_owned();
                            break 'inner;
                        }
                    }
                    Rule::A(cond) => {
                        if cond(part.a) {
                            current_rule = next.to_owned();
                            break 'inner;
                        }
                    }
                    Rule::S(cond) => {
                        if cond(part.s) {
                            current_rule = next.to_owned();
                            break 'inner;
                        }
                    }
                    Rule::Pass => {
                        if next == "A" {
                            sum += part.x + part.m + part.a + part.s;
                            break 'outer;
                        } else if next == "R" {
                            break 'outer;
                        } else {
                            current_rule = next.to_owned();
                            break 'inner;
                        }
                    }
                }
            }
        }
    }

    assert_eq!(sum, 420739);
}
