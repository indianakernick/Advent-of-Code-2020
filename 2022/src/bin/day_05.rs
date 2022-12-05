use advent_of_code_2022 as util;
use text_io::read;

fn main() {
    /*
    [F]         [L]     [M]
    [T]     [H] [V] [G] [V]
    [N]     [T] [D] [R] [N]     [D]
    [Z]     [B] [C] [P] [B] [R] [Z]
    [M]     [J] [N] [M] [F] [M] [V] [H]
    [G] [J] [L] [J] [S] [C] [G] [M] [F]
    [H] [W] [V] [P] [W] [H] [H] [N] [N]
    [J] [V] [G] [B] [F] [G] [D] [H] [G]
     1   2   3   4   5   6   7   8   9
    */

    let mut crates = vec![
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

    let mut index = 0;

    util::each_line("input/day_05.txt", |line| {
        if index <= 9 {
            index += 1;
            return;
        }

        let mut line_iter = line.bytes();

        let count: usize = read!("move {} from ", line_iter);
        let from: usize = read!("{} to ", line_iter);
        let to: usize = read!("{}", line_iter);
        let from = from - 1;
        let to = to - 1;

        for _ in 0..count {
            let top = crates[from].pop().unwrap();
            crates[to].push(top);
        }
    });

    print!("Part 1: ");
    for crate_ in crates.iter() {
        print!("{}", crate_.last().unwrap());
    }
    println!();
}
