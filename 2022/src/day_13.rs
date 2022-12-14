use std::cmp::Ordering;

enum PacketToken {
    Int(i32),
    Open,
    Close,
}

struct Packet(Vec<PacketToken>);

impl Packet {
    fn divider(val: i32) -> Self {
        use PacketToken::*;
        Self(vec![Open, Open, Int(val), Close, Close])
    }
}

fn compare<'l, 'r>(mut left: &'l[PacketToken], mut right: &'r[PacketToken])
    -> (Ordering, &'l[PacketToken], &'r[PacketToken])
{
    use PacketToken::*;

    match (left.is_empty(), right.is_empty()) {
        (true, true) => return (Ordering::Equal, left, right),
        (true, false) => return (Ordering::Less, left, right),
        (false, true) => return (Ordering::Greater, left, right),
        (false, false) => {}
    }

    match (&left[0], &right[0]) {
        (Int(l), Int(r)) => (l.cmp(&r), &left[1..], &right[1..]),

        (Int(_), Open) => {
            if let Close = right[1] {
                (Ordering::Greater, left, right)
            } else {
                let (order, left_rest, right_rest) = compare(left, &right[1..]);

                if !order.is_eq() {
                    return (order, left, right);
                }

                if let Close = right_rest[0] {
                    (Ordering::Equal, left_rest, &right_rest[1..])
                } else {
                    (Ordering::Less, left, right)
                }
            }
        },

        (Open, Int(_)) => {
            if let Close = left[1] {
                (Ordering::Less, left, right)
            } else {
                let (order, left_rest, right_rest) = compare(&left[1..], right);

                if !order.is_eq() {
                    return (order, left, right);
                }

                if let Close = left_rest[0] {
                    (Ordering::Equal, &left_rest[1..], right_rest)
                } else {
                    (Ordering::Greater, left, right)
                }
            }
        }

        (Open, Open) => {
            left = &left[1..];
            right = &right[1..];

            loop {
                match (&left[0], &right[0]) {
                    (Close, Close) => return (Ordering::Equal, &left[1..], &right[1..]),
                    (Close, _) => return (Ordering::Less, left, right),
                    (_, Close) => return (Ordering::Greater, left, right),
                    _ => {}
                }

                let (order, left_rest, right_rest) = compare(left, right);

                if !order.is_eq() {
                    return (order, left, right);
                }
                left = left_rest;
                right = right_rest;
            }
        },

        _ => panic!("Invalid packet"),
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
        compare(&self.0, &other.0).0
    }
}

impl Eq for Packet {}

fn parse(mut line: &str) -> Packet {
    let mut packet = Vec::new();

    while !line.is_empty() {
        if let Some(rest) = line.strip_prefix('[') {
            packet.push(PacketToken::Open);
            line = rest;
            continue;
        }

        if let Some(rest) = line.strip_prefix(']') {
            packet.push(PacketToken::Close);
            line = rest;
            continue;
        }

        if let Some(rest) = line.strip_prefix(',') {
            line = rest;
            continue;
        }

        if let Some(end) = line.find(|c: char| !c.is_ascii_digit()) {
            packet.push(PacketToken::Int(line[..end].parse().unwrap()));
            line = &line[end..];
        } else {
            packet.push(PacketToken::Int(line.parse().unwrap()));
            break;
        }
    }

    Packet(packet)
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

        let left = parse(left);
        let right = parse(right);

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
