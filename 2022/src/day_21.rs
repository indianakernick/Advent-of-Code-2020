use std::collections::HashMap;

type MonkeyId = u32;

const fn to_id(b: &[u8]) -> MonkeyId {
    ((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | b[3] as u32
}

const ROOT: MonkeyId = to_id(&[b'r', b'o', b'o', b't']);
const HUMAN: MonkeyId = to_id(&[b'h', b'u', b'm', b'n']);

#[derive(Clone, Copy)]
enum Monkey {
    Var(i64),
    Const(i64),
    Add(MonkeyId, MonkeyId),
    Sub(MonkeyId, MonkeyId),
    Mul(MonkeyId, MonkeyId),
    Div(MonkeyId, MonkeyId),
}

struct EvalResult {
    value: i64,
    variable: bool
}

impl EvalResult {
    fn new(value: i64, variable: bool) -> Self {
        Self { value, variable }
    }
}

fn fold_pair<F: Fn(i64, i64) -> i64>(
    map: &mut HashMap::<MonkeyId, Monkey>,
    lhs: MonkeyId,
    rhs: MonkeyId,
    op: F,
) -> EvalResult {
    let lhs = fold(map, lhs);
    let rhs = fold(map, rhs);
    let value = op(lhs.value, rhs.value);
    let variable = lhs.variable || rhs.variable;
    EvalResult::new(value, variable)
}

fn fold(map: &mut HashMap<MonkeyId, Monkey>, id: MonkeyId) -> EvalResult {
    let res = match map.get(&id).unwrap() {
        Monkey::Var(v) => EvalResult::new(*v, true),
        Monkey::Const(v) => EvalResult::new(*v, false),
        Monkey::Add(a, b) => fold_pair(map, *a, *b, |a, b| a + b),
        Monkey::Sub(a, b) => fold_pair(map, *a, *b, |a, b| a - b),
        Monkey::Mul(a, b) => fold_pair(map, *a, *b, |a, b| a * b),
        Monkey::Div(a, b) => fold_pair(map, *a, *b, |a, b| a / b),
    };

    if !res.variable {
        map.insert(id, Monkey::Const(res.value));
    }

    res
}

fn search(map: &HashMap<MonkeyId, Monkey>, id: MonkeyId, target: i64) -> i64 {
    match map.get(&id).unwrap() {
        Monkey::Var(_) => target,
        Monkey::Const(_) => panic!(),
        Monkey::Add(lhs_id, rhs_id) => {
            let lhs = map.get(lhs_id).unwrap();
            let rhs = map.get(rhs_id).unwrap();
            match (lhs, rhs) {
                (Monkey::Const(lhs_value), _) => search(map, *rhs_id, target - lhs_value),
                (_, Monkey::Const(rhs_value)) => search(map, *lhs_id, target - rhs_value),
                _ => panic!()
            }
        }
        Monkey::Sub(lhs_id, rhs_id) => {
            let lhs = map.get(lhs_id).unwrap();
            let rhs = map.get(rhs_id).unwrap();
            match (lhs, rhs) {
                (Monkey::Const(lhs_value), _) => search(map, *rhs_id, lhs_value - target),
                (_, Monkey::Const(rhs_value)) => search(map, *lhs_id, target + rhs_value),
                _ => panic!()
            }
        }
        Monkey::Mul(lhs_id, rhs_id) => {
            let lhs = map.get(lhs_id).unwrap();
            let rhs = map.get(rhs_id).unwrap();
            match (lhs, rhs) {
                (Monkey::Const(lhs_value), _) => search(map, *rhs_id, target / lhs_value),
                (_, Monkey::Const(rhs_value)) => search(map, *lhs_id, target / rhs_value),
                _ => panic!()
            }
        }
        Monkey::Div(lhs_id, rhs_id) => {
            let lhs = map.get(lhs_id).unwrap();
            let rhs = map.get(rhs_id).unwrap();
            match (lhs, rhs) {
                (Monkey::Const(lhs_value), _) => search(map, *rhs_id, lhs_value / target),
                (_, Monkey::Const(rhs_value)) => search(map, *lhs_id, target * rhs_value),
                _ => panic!()
            }
        }
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    let mut monkeys = HashMap::<MonkeyId, Monkey>::new();

    for line in input.lines() {
        let bytes = line.as_bytes();
        let id = to_id(&bytes[..4]);
        let monkey: Monkey;

        if bytes[6].is_ascii_digit() {
            if id == HUMAN {
                monkey = Monkey::Var(line[6..].parse().unwrap());
            } else {
                monkey = Monkey::Const(line[6..].parse().unwrap());
            }
        } else if bytes[11] == b'+' {
            monkey = Monkey::Add(to_id(&bytes[6..10]), to_id(&bytes[13..]));
        } else if bytes[11] == b'-' {
            monkey = Monkey::Sub(to_id(&bytes[6..10]), to_id(&bytes[13..]));
        } else if bytes[11] == b'*' {
            monkey = Monkey::Mul(to_id(&bytes[6..10]), to_id(&bytes[13..]));
        } else if bytes[11] == b'/' {
            monkey = Monkey::Div(to_id(&bytes[6..10]), to_id(&bytes[13..]));
        } else {
            panic!("Invalid input");
        }

        monkeys.insert(id, monkey);
    }

    let part_1 = fold(&mut monkeys, ROOT).value;

    let (lhs_id, rhs_id) = match monkeys.get(&ROOT).unwrap() {
        Monkey::Var(_) | Monkey::Const(_) => panic!(),
        Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => (*a, *b)
    };

    let lhs_monkey = monkeys.get(&lhs_id).unwrap();
    let rhs_monkey = monkeys.get(&rhs_id).unwrap();

    let part_2 = match (lhs_monkey, rhs_monkey) {
        (Monkey::Const(lhs), _) => search(&monkeys, rhs_id, *lhs),
        (_, Monkey::Const(rhs)) => search(&monkeys, lhs_id, *rhs),
        _ => panic!("One of the operands must be constant"),
    };

    (part_1, part_2)
}
