use std::cmp::Ordering;

enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn divider(val: i32) -> Self {
        Packet::List(vec![Packet::List(vec![Packet::Int(val)])])
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(&r),
            (Packet::Int(l), Packet::List(r)) => {
                if r.is_empty() {
                    Ordering::Greater
                } else {
                    let order = Packet::Int(*l).cmp(&r[0]);
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
                    let order = l[0].cmp(&Packet::Int(*r));
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
                    let order = l[i].cmp(&r[i]);
                    if !order.is_eq() {
                        return order;
                    }
                }
                l.len().cmp(&r.len())
            },
        }
    }
}

impl Eq for Packet {}

fn parse(line: &str) -> (Packet, &str) {
    if let Some(mut line) = line.strip_prefix('[') {
        let mut list = Vec::new();

        loop {
            if let Some(rest) = line.strip_prefix(']') {
                line = rest;
                break;
            }
            let (inner, rest) = parse(line);
            list.push(inner);
            line = rest;
            if let Some(rest) = line.strip_prefix(',') {
                line = rest;
            }
        }

        (Packet::List(list), line)
    } else {
        if let Some(end) = line.find(|c: char| !c.is_ascii_digit()) {
            (Packet::Int(line[..end].parse().unwrap()), &line[end..])
        } else {
            (Packet::Int(line.parse().unwrap()), "")
        }
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut line_iter = input.lines();
    let mut ordered_sum = 0;
    let mut pair_index = 1;
    let mut packets = Vec::new();

    loop {
        let Some(left) = line_iter.next() else {
            break;
        };
        let right = line_iter.next().unwrap();

        line_iter.next();

        let left = parse(left).0;
        let right = parse(right).0;

        if left < right {
            ordered_sum += pair_index;
        }

        packets.push(left);
        packets.push(right);

        pair_index += 1;
    }

    packets.sort_unstable();

    let divider_a = 1 + packets.binary_search(&Packet::divider(2)).unwrap_err();
    let divider_b = 2 + packets.binary_search(&Packet::divider(6)).unwrap_err();

    (ordered_sum, divider_a * divider_b)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    let output = solve(input);
    assert_eq!(output.0, 13);
    assert_eq!(output.1, 140);
}
