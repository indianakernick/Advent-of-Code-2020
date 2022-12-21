use std::collections::HashMap;

enum Monkey {
    Leaf(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String)
}

fn evaluate(map: &HashMap::<String, Monkey>, name: &str) -> i64 {
    match map.get(name).unwrap() {
        Monkey::Leaf(n) => *n,
        Monkey::Add(a, b) => evaluate(map, a) + evaluate(map, b),
        Monkey::Sub(a, b) => evaluate(map, a) - evaluate(map, b),
        Monkey::Mul(a, b) => evaluate(map, a) * evaluate(map, b),
        Monkey::Div(a, b) => evaluate(map, a) / evaluate(map, b),
    }
}

pub fn solve(input: &str) -> (i64, usize) {
    let mut monkeys = HashMap::<String, Monkey>::new();

    for line in input.lines() {
        let bytes = line.as_bytes();
        let name = &line[..4];
        let monkey: Monkey;

        if bytes[6].is_ascii_digit() {
            monkey = Monkey::Leaf(line[6..].parse().unwrap());
        } else if bytes[11] == b'+' {
            monkey = Monkey::Add(line[6..10].into(), line[13..].into());
        } else if bytes[11] == b'-' {
            monkey = Monkey::Sub(line[6..10].into(), line[13..].into());
        } else if bytes[11] == b'*' {
            monkey = Monkey::Mul(line[6..10].into(), line[13..].into());
        } else if bytes[11] == b'/' {
            monkey = Monkey::Div(line[6..10].into(), line[13..].into());
        } else {
            panic!("Invalid input");
        }

        monkeys.insert(name.into(), monkey);
    }

    let value = evaluate(&monkeys, "root");

    (value, 0)
}