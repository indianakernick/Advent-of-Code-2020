use advent_of_code_2022 as util;
use text_io::scan;

fn stack_tops(stacks: &[Vec<u8>]) -> String {
    let mut s = String::new();

    for stack in stacks.iter() {
        if let Some(top) = stack.last() {
            s.push(*top as char);
        }
    }

    s
}

fn main() {
    let mut stacks_1 = Vec::<Vec<u8>>::new();
    let mut stacks_2 = Vec::<Vec<u8>>::new();
    let mut reading_stacks = true;

    util::each_line("input/day_05.txt", |line| {
        if reading_stacks {
            if line.starts_with(" 1") {
                return;
            }

            if line.is_empty() {
                reading_stacks = false;

                for stack in stacks_1.iter_mut() {
                    stack.reverse();
                }

                stacks_2 = stacks_1.clone();
                return;
            }

            let stack_count = (line.len() + 1) / 4;

            if stacks_1.len() < stack_count {
                stacks_1.resize_with(stack_count, Default::default);
            }

            let line_bytes = line.as_bytes();
            let mut index = 0;

            while index < line_bytes.len() - 2 {
                if line_bytes[index] == b'[' {
                    stacks_1[index / 4].push(line_bytes[index + 1]);
                }
                index += 4;
            }
            return;
        }

        let count: usize;
        let from: usize;
        let to: usize;
        scan!(line.bytes() => "move {} from {} to {}", count, from, to);
        let from = from - 1;
        let to = to - 1;

        if from == to {
            return;
        }

        for _ in 0..count {
            let top = stacks_1[from].pop().unwrap();
            stacks_1[to].push(top);
        }

        let (from_stack, to_stack) = if from < to {
            let (from_slice, to_slice) = stacks_2.split_at_mut(to);
            (&mut from_slice[from], &mut to_slice[0])
        } else {
            let (to_slice, from_slice) = stacks_2.split_at_mut(from);
            (&mut from_slice[0], &mut to_slice[to])
        };
        let new_size = from_stack.len() - count;

        to_stack.extend_from_slice(&from_stack[new_size..]);
        from_stack.truncate(new_size);
    });

    println!("Part 1: {}", stack_tops(&stacks_1));
    println!("Part 2: {}", stack_tops(&stacks_2));
}
