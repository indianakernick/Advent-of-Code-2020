use advent_of_code_2022 as util;
use text_io::read;

fn stack_tops(stacks: &[Vec<char>]) -> String {
    let mut s = String::new();

    for stack in stacks.iter() {
        if let Some(top) = stack.last() {
            s.push(*top);
        }
    }

    s
}

fn main() {
    let mut stacks_1 = vec![
        vec!['J', 'H', 'G', 'M', 'Z', 'N', 'T', 'F'],
        vec!['V', 'W', 'J'],
        vec!['G', 'V', 'L', 'J', 'B', 'T', 'H'],
        vec!['B', 'P', 'J', 'N', 'C', 'D', 'V', 'L'],
        vec!['F', 'W', 'S', 'M', 'P', 'R', 'G'],
        vec!['G', 'H', 'C', 'F', 'B', 'N', 'V', 'M'],
        vec!['D', 'H', 'G', 'M', 'R'],
        vec!['H', 'N', 'M', 'V', 'Z', 'D'],
        vec!['G', 'N', 'F', 'H'],
    ];
    let mut stacks_2 = stacks_1.clone();
    let mut skip = true;

    util::each_line("input/day_05.txt", |line| {
        if skip {
            skip = !line.is_empty();
            return;
        }

        let mut line_iter = line.bytes();

        let count: usize = read!("move {} from ", line_iter);
        let from: usize = read!("{} to ", line_iter);
        let to: usize = read!("{}", line_iter);
        let from = from - 1;
        let to = to - 1;

        for _ in 0..count {
            let top = stacks_1[from].pop().unwrap();
            stacks_1[to].push(top);
        }

        let new_size = stacks_2[from].len() - count;
        let top = Vec::from(&stacks_2[from][new_size..]);
        stacks_2[to].extend(top.iter());
        stacks_2[from].truncate(new_size);
    });

    println!("Part 1: {}", stack_tops(&stacks_1));
    println!("Part 2: {}", stack_tops(&stacks_2));
}
