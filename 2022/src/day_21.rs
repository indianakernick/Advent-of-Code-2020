use std::collections::HashMap;

type MonkeyId = u32;

const fn to_id(b: &[u8]) -> MonkeyId {
    ((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | b[3] as u32
}

const ROOT: MonkeyId = to_id(b"root");
const HUMAN: MonkeyId = to_id(b"humn");

#[derive(Clone, Copy)]
enum Monkey {
    Var(i64),
    Const(i64),
    Add(MonkeyId, MonkeyId),
    Sub(MonkeyId, MonkeyId),
    Mul(MonkeyId, MonkeyId),
    Div(MonkeyId, MonkeyId),
}

type MonkeyMap = HashMap<MonkeyId, Monkey>;

struct FoldResult {
    value: i64,
    variable: bool
}

fn fold_pair<F: Fn(i64, i64) -> i64>(
    map: &mut MonkeyMap,
    lhs: MonkeyId,
    rhs: MonkeyId,
    op: F,
) -> FoldResult {
    let lhs = fold(map, lhs);
    let rhs = fold(map, rhs);
    FoldResult {
        value: op(lhs.value, rhs.value),
        variable: lhs.variable || rhs.variable
    }
}

fn fold(map: &mut MonkeyMap, id: MonkeyId) -> FoldResult {
    let res = match map.get(&id).unwrap() {
        Monkey::Var(v) => FoldResult { value: *v, variable: true },
        Monkey::Const(v) => FoldResult { value: *v, variable: false },
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

fn search_pair<L, R>(
    map: &MonkeyMap,
    lhs_id: MonkeyId,
    rhs_id: MonkeyId,
    eval_lhs: L,
    eval_rhs: R,
) -> i64
    where L: Fn(i64) -> i64, R: Fn(i64) -> i64
{
    let lhs = map.get(&lhs_id).unwrap();
    let rhs = map.get(&rhs_id).unwrap();
    match (lhs, rhs) {
        (Monkey::Const(lhs_value), _) => search(map, rhs_id, eval_lhs(*lhs_value)),
        (_, Monkey::Const(rhs_value)) => search(map, lhs_id, eval_rhs(*rhs_value)),
        _ => panic!()
    }
}

fn search(map: &MonkeyMap, id: MonkeyId, target: i64) -> i64 {
    match map.get(&id).unwrap() {
        Monkey::Var(_) => target,
        Monkey::Const(_) => panic!(),
        Monkey::Add(lhs, rhs) => {
            search_pair(map, *lhs, *rhs, |l| target - l, |r| target - r)
        }
        Monkey::Sub(lhs, rhs) => {
            search_pair(map, *lhs, *rhs, |l| l - target, |r| target + r)
        }
        Monkey::Mul(lhs, rhs) => {
            search_pair(map, *lhs, *rhs, |l| target / l, |r| target / r)
        }
        Monkey::Div(lhs, rhs) => {
            search_pair(map, *lhs, *rhs, |l| l / target, |r| target * r)
        }
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    let mut monkeys = MonkeyMap::new();

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
        } else {
            let lhs = to_id(&bytes[6..10]);
            let rhs = to_id(&bytes[13..]);
            monkey = match bytes[11] {
                b'+' => Monkey::Add(lhs, rhs),
                b'-' => Monkey::Sub(lhs, rhs),
                b'*' => Monkey::Mul(lhs, rhs),
                b'/' => Monkey::Div(lhs, rhs),
                _ => panic!("Invalid input"),
            };
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

#[cfg(test)]
#[test]
fn example() {
    let input =
"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    let output = solve(input);
    assert_eq!(output.0, 152);
    assert_eq!(output.1, 301);
}
