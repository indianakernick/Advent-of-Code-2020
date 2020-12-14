use adventofcode2020::*;

#[derive(Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}

enum Exit {
    Normal(i32),
    Loop(i32)
}

fn parse_instr(line: &str) -> Instruction {
    let argument = line[4..].parse::<i32>().unwrap();
    match &line[..4] {
        "acc " => Instruction::Acc(argument),
        "jmp " => Instruction::Jmp(argument),
        "nop " => Instruction::Nop(argument),
        _ => panic!()
    }
}

fn parse_input() -> Vec<Instruction> {
    let mut instrs = Vec::new();
    lines_from_file("input/day_8.txt", |line| {
        instrs.push(parse_instr(line));
    });
    instrs
}

fn interpret(instrs: &Vec<Instruction>) -> Exit {
    let mut visited = vec![false; instrs.len()];
    let mut pc = 0;
    let mut acc = 0;

    loop {
        if pc == instrs.len() {
            return Exit::Normal(acc);
        }
        if visited[pc] {
            return Exit::Loop(acc);
        }
        visited[pc] = true;

        match instrs[pc] {
            Instruction::Acc(arg) => { acc += arg; pc += 1 },
            Instruction::Jmp(arg) => pc = pc.wrapping_add(arg as usize),
            Instruction::Nop(_) => pc += 1
        }
    }
}

fn acc_after_loop(instrs: &Vec<Instruction>) -> i32 {
    match interpret(instrs) {
        Exit::Loop(acc) => acc,
        Exit::Normal(_) => panic!()
    }
}

fn acc_after_fix(instrs: &mut Vec<Instruction>) -> i32 {
    for i in 0..instrs.len() {
        let old_instr = instrs[i].clone();
        let new_instr = match instrs[i] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(arg) => Instruction::Nop(arg),
            Instruction::Nop(arg) => Instruction::Jmp(arg)
        };

        instrs[i] = new_instr;
        match interpret(&instrs) {
            Exit::Normal(acc) => return acc,
            Exit::Loop(_) => instrs[i] = old_instr
        }
    }

    return 0;
}

fn main() {
    let mut instrs = parse_input();
    println!("Part one: {}", acc_after_loop(&instrs));
    println!("Part two: {}", acc_after_fix(&mut instrs));
}
