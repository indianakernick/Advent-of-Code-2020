use adventofcode2020::*;

fn part_one() {
    let mut ones_mask: u64 = 0;
    let mut zeros_mask: u64 = 0xFFFFFFFFFFFFFFFF;
    let mut memory = Vec::<u64>::new();

    lines_from_file("input/day_14.txt", |line| {
        if line.starts_with("mask = ") {
            ones_mask = 0;
            zeros_mask = 0xFFFFFFFFFFFFFFFF;
            for i in 0..36 {
                match line.chars().nth(7 + 36 - i - 1).unwrap() {
                    'X' => {},
                    '1' => ones_mask |= 1 << i,
                    '0' => zeros_mask &= !(1 << i),
                    _ => panic!()
                }
            }
        } else if line.starts_with("mem[") {
            let address: usize;
            let value: u64;
            scan!(line.bytes() => "mem[{}] = {}", address, value);
            if address >= memory.len() {
                memory.resize(address + 1, 0);
            }
            memory[address] = (value | ones_mask) & zeros_mask;
        }
    });

    let mut sum = 0;
    for val in memory {
        sum += val;
    }
    println!("Part one: {}", sum);
}

fn part_two() {
    let mut ones_mask: usize = 0;
    let mut floating_bits = Vec::<usize>::new();
    let mut memory = std::collections::hash_map::HashMap::<usize, u64>::new();

    lines_from_file("input/day_14.txt", |line| {
        if line.starts_with("mask = ") {
            ones_mask = 0;
            floating_bits.clear();
            for i in 0..36 {
                match line.chars().nth(7 + 36 - i - 1).unwrap() {
                    '0' => {},
                    '1' => ones_mask |= 1 << i,
                    'X' => floating_bits.push(i),
                    _ => panic!()
                }
            }
        } else if line.starts_with("mem[") {
            let mut address: usize;
            let value: u64;
            scan!(line.bytes() => "mem[{}] = {}", address, value);
            address = address | ones_mask;
            for i in 0..(1 << floating_bits.len()) {
                for j in 0..floating_bits.len() {
                    address &= !(1 << floating_bits[j]);
                    if (i >> j) & 1 == 1 {
                        address |= 1 << floating_bits[j];
                    }
                }
                memory.insert(address, value);
            }
        }
    });

    let mut sum = 0;
    for (addr, val) in memory {
        sum += val;
    }
    println!("Part two: {}", sum);
}

fn main() {
    part_one();
    part_two();
}
