use adventofcode2020::*;

fn part_one(line: &String, start: usize) -> (u64, usize) {
    let mut value: u64 = 0;
    let mut op = 0;
    let mut idx = start;
    while idx < line.chars().count() {
        let ch = line.chars().nth(idx).unwrap();
        if ch == ' ' { idx += 1; continue; }
        if ch == ')' { idx += 1; break; }
        if ch.is_ascii_digit() {
            let digit = ch.to_digit(10).unwrap() as u64;
            if op == 0 {
                value = digit;
            } else if op == 1 {
                value += digit;
            } else if op == 2 {
                value *= digit;
            }
        } else if ch == '+' {
            op = 1;
        } else if ch == '*' {
            op = 2;
        } else if ch == '(' {
            let (inner, count) = part_one(line, idx + 1);
            idx += count;
            if op == 0 {
                value = inner;
            } else if op == 1 {
                value += inner;
            } else if op == 2 {
                value *= inner;
            }
        }
        idx += 1;
    }
    (value, idx - start)
}

fn part_two(line: &String) -> u64 {
    let mut idx = 0;
    let mut output_queue = std::collections::VecDeque::new();
    let mut oper_stack = Vec::new();

    while idx < line.chars().count() {
        let ch = line.chars().nth(idx).unwrap();
        if ch == ' ' { idx += 1; continue; }

        if ch.is_ascii_digit() {
            let digit = ch.to_digit(10).unwrap() as u64;
            output_queue.push_back(digit);
        } else if ch == '+' { // 2
            oper_stack.push(2);
        } else if ch == '*' { // 1
            while !oper_stack.is_empty() && *oper_stack.last().unwrap() == 2 {
                output_queue.push_back(oper_stack.pop().unwrap() + 10);
            }
            oper_stack.push(1);
        } else if ch == '(' {
            oper_stack.push(3);
        } else if ch == ')' {
            while *oper_stack.last().unwrap() != 3 {
                output_queue.push_back(oper_stack.pop().unwrap() + 10);
            }
            if *oper_stack.last().unwrap() == 3 {
                oper_stack.pop();
            }
        }
        idx += 1;
    }

    while !oper_stack.is_empty() {
        output_queue.push_back(oper_stack.pop().unwrap() + 10);
    }

    let mut eval_stack = Vec::new();
    while !output_queue.is_empty() {
        let item = output_queue.pop_front().unwrap();
        if item == 11 {
            let a = eval_stack.pop().unwrap();
            let b = eval_stack.pop().unwrap();
            eval_stack.push(a * b);
        } else if item == 12 {
            let a = eval_stack.pop().unwrap();
            let b = eval_stack.pop().unwrap();
            eval_stack.push(a + b);
        } else if item == 13 {
            panic!();
        } else {
            eval_stack.push(item);
        }
    }

    *eval_stack.first().unwrap()
}

fn main() {
    let mut sum_one = 0;
    let mut sum_two = 0;
    lines_from_file("input/day_18.txt", |line| {
        sum_one += part_one(line, 0).0;
        sum_two += part_two(line);
    });
    println!("Part one: {}", sum_one);
    println!("Part two: {}", sum_two);
}
