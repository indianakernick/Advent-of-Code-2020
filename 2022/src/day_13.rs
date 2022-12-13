use std::cmp::Ordering;

#[derive(Clone)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

fn parse(line: &str) -> (Packet, &str) {
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

        (Packet::List(list), line)
    } else {
        if let Some(end) = line.find(|c: char| !c.is_ascii_digit()) {
            (Packet::Int(line[0..end].parse().unwrap()), &line[end..])
        } else {
            (Packet::Int(line.parse().unwrap()), "")
        }
    }
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Int(l), Packet::Int(r)) => l.cmp(&r),
        (Packet::Int(l), Packet::List(r)) => {
            if r.is_empty() {
                Ordering::Greater
            } else {
                let order = compare(&Packet::Int(*l), &r[0]);
                if !order.is_eq() {
                    return order;
                }
                if r.len() > 1 {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        },
        (Packet::List(l), Packet::Int(r)) => {
            if l.is_empty() {
                Ordering::Less
            } else {
                let order = compare(&l[0], &Packet::Int(*r));
                if !order.is_eq() {
                    return order;
                }
                if l.len() > 1 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        },
        (Packet::List(l), Packet::List(r)) => {
            for i in 0..l.len().min(r.len()) {
                let order = compare(&l[i], &r[i]);
                if !order.is_eq() {
                    return order;
                }
            }
            l.len().cmp(&r.len())
        },
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut line_iter = input.lines();
    let mut ordered_sum = 0;
    let mut pair_index = 1;
    let mut packets = Vec::new();

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

        if compare(&left, &right) == Ordering::Less {
            ordered_sum += pair_index;
        }

        packets.push(left);
        packets.push(right);

        pair_index += 1;
    }

    let decoder_a = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let decoder_b = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packets.push(decoder_a.clone());
    packets.push(decoder_b.clone());
    packets.sort_unstable_by(|a, b| compare(a, b));

    let idx_a = 1 + packets.iter().position(|p| compare(&decoder_a, p).is_eq()).unwrap();
    let idx_b = 1 + packets.iter().position(|p| compare(&decoder_b, p).is_eq()).unwrap();

    (ordered_sum, idx_a * idx_b)
}
