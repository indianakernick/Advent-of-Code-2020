use adventofcode2020::*;

enum Command {
    SetMask { ones: u64, zeros: u64 },
    SetMemory { address: u64, value: u64 }
}

type Memory = std::collections::HashMap::<u64, u64>;

fn parse_input() -> Vec::<Command> {
    line_iter_from_file("input/day_14.txt")
        .map(|line| {
            let line_bytes = line.as_bytes();
            if line_bytes.starts_with(b"mask = ") {
                let mut ones = 0;
                let mut zeros = 0;
                for bit in 0..36 {
                    match line_bytes[line_bytes.len() - 1 - bit] {
                        b'0' => zeros |= 1 << bit,
                        b'1' => ones |= 1 << bit,
                        _ => {}
                    }
                }
                Command::SetMask { ones, zeros }
            } else {
                let address: u64;
                let value: u64;
                // line.bytes() works but line.as_bytes().iter() doesn't.
                // Rust is hard
                scan!(line.bytes() => "mem[{}] = {}", address, value);
                Command::SetMemory { address, value }
            }
        })
        .collect()
}

fn memory_sum(memory: Memory) -> u64 {
    memory.iter().map(|(_, value)| value).sum()
}

fn part_one(commands: &Vec::<Command>) -> u64 {
    let mut ones_mask: u64 = 0;
    let mut zeros_mask: u64 = 0xFFFFFFFFFFFFFFFF;
    let mut memory = Memory::new();

    for command in commands {
        match command {
            Command::SetMask { ones, zeros } => {
                ones_mask = *ones;
                zeros_mask = !*zeros;
            },
            Command::SetMemory { address, value } => {
                memory.insert(*address, (value | ones_mask) & zeros_mask);
            }
        }
    }

    memory_sum(memory)
}

fn part_two(commands: &Vec::<Command>) -> u64 {
    let mut ones_mask: u64 = 0;
    let mut floating_bits = Vec::<u8>::new();
    let mut memory = Memory::new();

    for command in commands {
        match command {
            Command::SetMask { ones, zeros } => {
                ones_mask = *ones;
                let xes = !(ones | zeros);
                floating_bits.clear();
                for bit in 0..36 {
                    if (xes >> bit) & 1 == 1 {
                        floating_bits.push(bit);
                    }
                }
            },
            Command::SetMemory { address, value } => {
                let mut address = *address;
                address |= ones_mask;
                for power_set_elem in 0..(1 << floating_bits.len()) {
                    for bit in 0..floating_bits.len() {
                        address &= !(1 << floating_bits[bit]);
                        address |= ((power_set_elem >> bit) & 1) << floating_bits[bit];
                    }
                    memory.insert(address, *value);
                }
            }
        }
    }

    memory_sum(memory)
}

fn main() {
    let commands = parse_input();
    println!("Part one: {}", part_one(&commands));
    println!("Part two: {}", part_two(&commands));
}
