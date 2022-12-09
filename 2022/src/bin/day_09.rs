use std::collections::HashSet;
use std::cmp::Ordering;
use advent_of_code_2022 as util;

fn main() {
    let mut rope: [(i32, i32); 10] = Default::default();
    let mut visits_1 = HashSet::<(i32, i32)>::new();
    let mut visits_9 = HashSet::<(i32, i32)>::new();

    visits_1.insert((0, 0));
    visits_9.insert((0, 0));

    util::each_line("input/day_09.txt", |line| {
        let direction = line.as_bytes()[0];
        let distance: u32 = line[2..].parse().unwrap();

        let dir_vec = match direction {
            b'U' => (0, -1),
            b'R' => (1, 0),
            b'D' => (0, 1),
            b'L' => (-1, 0),
            _ => panic!("Invalid direction")
        };

        for _ in 0..distance {
            rope[0].0 += dir_vec.0;
            rope[0].1 += dir_vec.1;

            for i in 1..rope.len() {
                let head = rope[i - 1];
                let tail = &mut rope[i];
                let tail_to_head = (head.0 - tail.0, head.1 - tail.1);

                if tail_to_head.0.abs() < 2 && tail_to_head.1.abs() < 2 {
                    continue;
                }

                match (tail_to_head.0.cmp(&0), tail_to_head.1.cmp(&0)) {
                    (Ordering::Less, Ordering::Less) => { tail.0 -= 1; tail.1 -= 1 },
                    (Ordering::Less, Ordering::Equal) => { tail.0 -= 1 },
                    (Ordering::Less, Ordering::Greater) => { tail.0 -= 1; tail.1 += 1 },
                    (Ordering::Equal, Ordering::Less) => { tail.1 -= 1 },
                    (Ordering::Equal, Ordering::Equal) => continue,
                    (Ordering::Equal, Ordering::Greater) => { tail.1 += 1 },
                    (Ordering::Greater, Ordering::Less) => { tail.0 += 1; tail.1 -= 1 },
                    (Ordering::Greater, Ordering::Equal) => { tail.0 += 1 },
                    (Ordering::Greater, Ordering::Greater) => { tail.0 += 1; tail.1 += 1 },
                }
            }

            visits_1.insert(rope[1]);
            visits_9.insert(rope[9]);
        }
    });

    println!("Part 1: {}", visits_1.len());
    println!("Part 2: {}", visits_9.len());
}
