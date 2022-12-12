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
