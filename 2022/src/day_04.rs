use text_io::scan;

pub fn solve(input: &str) -> (u64, u64) {
    let mut contain_count = 0u64;
    let mut overlap_count = 0u64;

    for line in input.lines() {
        let first_low: u32;
        let first_high: u32;
        let second_low: u32;
        let second_high: u32;
        scan!(line.bytes() => "{}-{},{}-{}", first_low, first_high, second_low, second_high);

        if (first_low >= second_low && first_high <= second_high)
            || (second_low >= first_low && second_high <= first_high) {
            contain_count += 1;
        }

        if (first_low <= second_high && first_high >= second_low)
            || (second_low <= first_high && second_high >= first_low) {
            overlap_count += 1;
        }
    }

    (contain_count, overlap_count)
}
