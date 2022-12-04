use adventofcode2020::*;
use std::collections::HashMap;

enum Rule {
    Char(char),
    Rules(Vec<Vec<u32>>)
}

fn parse_rule(rule: &str) -> Rule {
    if rule.len() == 3 && rule.chars().nth(0).unwrap() == '"' {
        Rule::Char(rule.chars().nth(1).unwrap())
    } else {
        let mut rules = Vec::new();
        for sub_rule in rule.split(" | ") {
            rules.push(sub_rule
                .split(" ")
                .map(|r| r.parse::<u32>().unwrap())
                .collect()
            );
        }
        Rule::Rules(rules)
    }
}

fn matches(rules: &HashMap::<u32, Rule>, line: &str, mut rule: Vec::<u32>) -> bool {
    if rule.is_empty() {
        return line.is_empty();
    }
    match &rules[&rule.remove(0)] {
        Rule::Char(ch) => {
            if line.len() == 0 {
                return false;
            }
            line.chars().nth(0).unwrap() == *ch && matches(rules, &line[1..], rule)
        },
        Rule::Rules(lower_rules) => {
            for lower_rule in lower_rules {
                let mut dup = lower_rule.clone();
                dup.append(&mut (rule.clone()));
                if matches(rules, line, dup) {
                    return true;
                }
            }
            return false;
        }
    }
}

fn parse_rules(line_iter: &mut impl Iterator<Item = String>) -> HashMap<u32, Rule> {
    line_iter
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let colon = line.find(':').unwrap();
            let rule_num = (&line[..colon]).parse::<u32>().unwrap();
            let rule_text = &line[(colon + 2)..];
            (rule_num, parse_rule(&rule_text))
        })
        .collect()
}

fn count_matches(rules: &HashMap<u32, Rule>, lines: &Vec<String>) -> u32 {
    let mut match_count = 0;
    for line in lines {
        if matches(&rules, &line, vec![0]) {
            match_count += 1;
        }
    }
    match_count
}

fn main() {
    let mut line_iter = line_iter_from_file("input/day_19.txt");
    let mut rules = parse_rules(&mut line_iter);
    let lines = line_iter.collect::<Vec<_>>();
    println!("{}", lines.len());
    println!("Part one: {}", count_matches(&rules, &lines));
    *rules.get_mut(&8).unwrap() = Rule::Rules(vec![vec![42], vec![42, 8]]);
    *rules.get_mut(&11).unwrap() = Rule::Rules(vec![vec![42, 31], vec![42, 11, 31]]);
    println!("Part two: {}", count_matches(&rules, &lines));
}
