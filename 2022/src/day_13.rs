use std::cmp::Ordering;

enum Lists {
    Int(i32),
    List(Vec<Lists>),
}

fn parse(line: &str) -> (Lists, &str) {
    if let Some(mut line) = line.strip_prefix('[') {
        let mut list = Vec::new();

        loop {
            if let Some(rest) = line.strip_prefix(']') {
                line = rest;
                break;
            }
            let (inner, rest) = parse(line);
            line = rest;
            list.push(inner);
            if let Some(rest) = line.strip_prefix(',') {
                line = rest;
            }
        }

        (Lists::List(list), line)
    } else {
        if let Some(end) = line.find(|c: char| !c.is_ascii_digit()) {
            (Lists::Int(line[0..end].parse().unwrap()), &line[end..])
        } else {
            (Lists::Int(line.parse().unwrap()), "")
        }
    }
}

fn compare(left: Lists, right: Lists) -> Ordering {
    match (left, right) {
        (Lists::Int(l), Lists::Int(r)) => l.cmp(&r),
        (Lists::Int(l), Lists::List(r)) => compare(Lists::List(vec![Lists::Int(l)]), Lists::List(r)),
        (Lists::List(l), Lists::Int(r)) => compare(Lists::List(l), Lists::List(vec![Lists::Int(r)])),
        (Lists::List(l), Lists::List(r)) => {
            let len_cmp = l.len().cmp(&r.len());
            for (li, ri) in l.into_iter().zip(r.into_iter()) {
                let order = compare(li, ri);
                if !order.is_eq() {
                    return order;
                }
            }
            len_cmp
        },
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut line_iter = input.lines();
    let mut ordered_sum = 0;
    let mut pair_index = 1;

    loop {
        let left = if let Some(l) = line_iter.next() {
            l
        } else {
            break;
        };
        let right = line_iter.next().unwrap();
        line_iter.next();

        let left = parse(left).0;
        let right = parse(right).0;

        if compare(left, right) == Ordering::Less {
            ordered_sum += pair_index;
        }

        pair_index += 1;
    }

    (ordered_sum, 0)
}
