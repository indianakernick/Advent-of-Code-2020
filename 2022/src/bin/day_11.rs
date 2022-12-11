enum Operation {
    Add(i32),
    Mul(i32),
    Square,
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    divisor: i32,
    test_true: usize,
    test_false: usize,
    inspection_count: usize,
}

fn main() {
    let input = std::fs::read_to_string("input/day_11.txt").unwrap();
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

    let mut throws = Vec::<(usize, i32)>::new();

    for r in 0..20 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.iter() {
                let new_worry = match monkeys[i].operation {
                    Operation::Add(add) => item + add,
                    Operation::Mul(mul) => item * mul,
                    Operation::Square => item * item,
                } / 3;
                let throw_target = if new_worry % monkeys[i].divisor == 0 {
                    monkeys[i].test_true
                } else {
                    monkeys[i].test_false
                };
                throws.push((throw_target, new_worry));
            }

            monkeys[i].items.clear();
            monkeys[i].inspection_count += throws.len();

            for (target, worry) in throws.iter() {
                monkeys[*target].items.push(*worry);
            }

            throws.clear();
        }
    }

    let mut counts = monkeys.iter().map(|m| m.inspection_count).collect::<Vec<_>>();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    println!("Part 1: {}", counts[0] * counts[1]);
}
