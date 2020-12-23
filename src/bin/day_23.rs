use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    value: i32,
    prev: usize,
    next: usize,
}

impl Node {
    fn new(value: i32, prev: usize, next: usize) -> Self {
        Node { value, prev, next }
    }
}

fn initialize() -> (Vec<Node>, HashMap<i32, usize>, i32) {
    let mut cups = vec![1, 6, 7, 2, 4, 8, 3, 5, 9];
    for i in 10..=1000000 {
        cups.push(i);
    }
    let mut cup_nodes = Vec::new();
    let mut idx_map = HashMap::new();
    for i in 0..cups.len() {
        let prev = (i + cups.len() - 1) % cups.len();
        let next = (i + 1) % cups.len();
        cup_nodes.push(Node::new(cups[i], prev, next));
        idx_map.insert(cups[i], i);
    }
    (cup_nodes, idx_map, 1000000)
}

fn main() {
    let (mut cup_nodes, idx_map, max_value) = initialize();
    let mut current_idx = 0;

    for _ in 0..10000000 {
        let taken_0 = cup_nodes[current_idx].next;
        let taken_1 = cup_nodes[taken_0].next;
        let taken_2 = cup_nodes[taken_1].next;
        let taken_end = cup_nodes[taken_2].next;
        cup_nodes[current_idx].next = taken_end;
        cup_nodes[taken_end].prev = current_idx;

        let taken_cups = [cup_nodes[taken_0].value, cup_nodes[taken_1].value, cup_nodes[taken_2].value];

        let mut destination = cup_nodes[current_idx].value - 1;
        while destination < 1 || taken_cups.contains(&destination) {
            destination -= 1;
            if destination < 1 {
                destination = max_value;
            }
        }

        let destination_idx = idx_map[&destination];
        let destination_end = cup_nodes[destination_idx].next;
        cup_nodes[destination_idx].next = taken_0;
        cup_nodes[taken_0].prev = destination_idx;
        cup_nodes[taken_2].next = destination_end;
        cup_nodes[destination_end].prev = taken_2;

        current_idx = cup_nodes[current_idx].next;
    }


    let mut idx = cup_nodes[idx_map[&1]].next;
    let first = cup_nodes[idx].value as u64;
    idx = cup_nodes[idx].next;
    let second = cup_nodes[idx].value as u64;
    println!("{}", first * second);
    /*for _ in 0..8 {
        print!("{}", cup_nodes[idx].value);
        idx = cup_nodes[idx].next;
    }*/
}