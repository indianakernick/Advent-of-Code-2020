#[derive(Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    test_true: usize,
    test_false: usize,
    inspection_count: usize,
}

fn process<const PART_1: bool>(mut monkeys: Vec<Monkey>, rounds: usize) -> usize {
    let mut throws = Vec::<(usize, u64)>::new();
    let modulus: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            for item in monkey.items.iter() {
                let mut new_worry = match monkey.operation {
                    Operation::Add(add) => item + add,
                    Operation::Mul(mul) => item * mul,
                    Operation::Square => item * item,
                };

                if PART_1 {
                    new_worry /= 3;
                } else {
                    new_worry %= modulus;
                }

                let throw_target = if new_worry % monkey.divisor == 0 {
                    monkey.test_true
                } else {
                    monkey.test_false
                };
                throws.push((throw_target, new_worry));
            }

            monkey.items.clear();
            monkey.inspection_count += throws.len();

            for (target, worry) in throws.iter() {
                monkeys[*target].items.push(*worry);
            }

            throws.clear();
        }
    }

    let mut counts = monkeys.iter().map(|m| m.inspection_count).collect::<Vec<_>>();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    counts[0] * counts[1]
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut monkeys = Vec::<Monkey>::new();
    let mut line_iter = input.lines();

    loop {
        if let Some(start) = line_iter.next() {
            if !start.starts_with("Monkey") {
                break;
            }
        } else {
            break;
        }

        let items = line_iter
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();

        let operation = line_iter
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap();
        let operation = if operation == "old * old" {
            Operation::Square
        } else if let Some(mul) = operation.strip_prefix("old * ") {
            Operation::Mul(mul.parse().unwrap())
        } else if let Some(add) = operation.strip_prefix("old + ") {
            Operation::Add(add.parse().unwrap())
        } else {
            panic!("Invalid operation");
        };

        let divisor = line_iter
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let test_true = line_iter
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let test_false = line_iter
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        line_iter.next();

        monkeys.push(Monkey {
            items, operation, divisor, test_true, test_false, inspection_count: 0
        });
    }

    (
        process::<true>(monkeys.clone(), 20),
        process::<false>(monkeys, 10000),
    )
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    let output = solve(input);
    assert_eq!(output.0, 10605);
    assert_eq!(output.1, 2713310158);
}
