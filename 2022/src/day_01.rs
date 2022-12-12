pub fn solve(input: &str) -> (u64, u64) {
    const PREFIX_COUNT: usize = 3;

    let mut max = Vec::with_capacity(PREFIX_COUNT);
    let mut curr = 0u64;

    for line in input.lines() {
        if line.is_empty() {
            if max.is_empty() {
                max.push(curr);
            } else if max[max.len() - 1] < curr {
                if max.len() == PREFIX_COUNT {
                    max.pop();
                }
                max.push(curr);
                max.sort_unstable_by(|a, b| b.cmp(a));
            }

            curr = 0;
        } else {
            curr += line.parse::<u64>().unwrap();
        }
    }

    (max[0], max.iter().sum::<u64>())
}
