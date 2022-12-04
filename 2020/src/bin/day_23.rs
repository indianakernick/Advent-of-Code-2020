use std::collections::HashMap;

struct Node {
    value: i32,
    prev: usize,
    next: usize,
}

fn initialize(part_two: bool) -> (Vec<Node>, HashMap<i32, usize>, i32) {
    let cups = vec![1, 6, 7, 2, 4, 8, 3, 5, 9];
    let length = if part_two { 1_000_000 } else { cups.len() };
    let mut cup_nodes = Vec::new();
    let mut idx_map = HashMap::new();

    for i in 0..cups.len() {
        let prev = (i + length - 1) % length;
        let next = (i + 1) % length;
        cup_nodes.push(Node { value: cups[i], prev, next });
        idx_map.insert(cups[i], i);
    }

    if part_two {
        for i in 9..length {
            let prev = i - 1;
            let next = (i + 1) % length;
            cup_nodes.push(Node { value: (i + 1) as i32, prev, next });
        }
    }

    (cup_nodes, idx_map, length as i32)
}

fn simulate(part_two: bool) {
    let (mut cups, idx_map, max_value) = initialize(part_two);
    let mut current_idx = 0;

    let moves = if part_two { 10_000_000 } else { 100 };

    for _ in 0..moves {
        let taken_0 = cups[current_idx].next;
        let taken_1 = cups[taken_0].next;
        let taken_2 = cups[taken_1].next;
        let taken_end = cups[taken_2].next;
        cups[current_idx].next = taken_end;
        cups[taken_end].prev = current_idx;

        let taken_cups = [
            cups[taken_0].value,
            cups[taken_1].value,
            cups[taken_2].value
        ];

        let mut destination = cups[current_idx].value - 1;
        while destination < 1 || taken_cups.contains(&destination) {
            destination -= 1;
            if destination < 1 {
                destination = max_value;
            }
        }

        let destination_idx = if destination < 10 {
            idx_map[&destination]
        } else {
            destination as usize - 1
        };
        let destination_end = cups[destination_idx].next;
        cups[destination_idx].next = taken_0;
        cups[taken_0].prev = destination_idx;
        cups[taken_2].next = destination_end;
        cups[destination_end].prev = taken_2;

        current_idx = cups[current_idx].next;
    }

    let mut idx = cups[idx_map[&1]].next;
    if part_two {
        let first = cups[idx].value as u64;
        idx = cups[idx].next;
        let second = cups[idx].value as u64;
        println!("Part two: {}", first * second);
    } else {
        print!("Part one: ");
        for _ in 0..8 {
            print!("{}", cups[idx].value);
            idx = cups[idx].next;
        }
        println!();
    }
}

fn main() {
    simulate(false);
    simulate(true);
}
