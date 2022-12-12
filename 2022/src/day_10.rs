fn run_cycle(reg: i32, cycle: &mut i32, strength: &mut i32, display: &mut [u8]) {
    let x = *cycle % 40;
    if x == reg || x == reg - 1 || x == reg + 1 {
        display[*cycle as usize] = b'#';
    }
    *cycle += 1;
    if *cycle >= 20 && (*cycle - 20) % 40 == 0 {
        *strength += *cycle * reg;
    }
}

pub fn solve(input: &str) -> (i32, String) {
    let mut reg = 1;
    let mut cycle = 0;
    let mut strength = 0;
    let mut display = vec![b'.'; 6 * 40];

    for line in input.lines() {
        if line == "noop" {
            run_cycle(reg, &mut cycle, &mut strength, &mut display);
        } else if let Some(num) = line.strip_prefix("addx ") {
            run_cycle(reg, &mut cycle, &mut strength, &mut display);
            run_cycle(reg, &mut cycle, &mut strength, &mut display);
            reg += num.parse::<i32>().unwrap();
        }
    }

    (
        strength,
        display.chunks(40)
            .map(|row| std::str::from_utf8(row).unwrap())
            .fold(String::new(), |s, row| {
                s + row + "\n"
            }),
    )
}

#[cfg(test)]
#[test]
fn example() {
    let input = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    let output = solve(input);
    assert_eq!(output.0, 13140);
    assert_eq!(output.1,
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
}
