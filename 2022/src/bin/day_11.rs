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

fn simulate<const PART_1: bool>(mut monkeys: Vec<Monkey>, rounds: usize) -> usize {
    let mut throws = Vec::<(usize, u64)>::new();

    let modulus: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.iter() {
                let mut new_worry: u64 = match monkeys[i].operation {
                    Operation::Add(add) => item + add,
                    Operation::Mul(mul) => item * mul,
                    Operation::Square => item * item
                };

                if PART_1 {
                    new_worry /= 3;
                } else {
                    new_worry %= modulus;
                }

                let throw_target = if new_worry % monkeys[i].divisor == 0 {
                    monkeys[i].test_true
                } else {
                    monkeys[i].test_false
                };
                throws.push((throw_target, new_worry));
            }

            monkeys[i].items.clear();
            monkeys[i].inspection_count += throws.len();

            for (target, worry) in throws.drain(..) {
                monkeys[target].items.push(worry);
            }
        }
    }

    let mut counts = monkeys.iter().map(|m| m.inspection_count).collect::<Vec<_>>();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    counts[0] * counts[1]
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

    println!("Part 1: {}", simulate::<true>(monkeys.clone(), 20));
    println!("Part 2: {}", simulate::<false>(monkeys, 10000));
}
