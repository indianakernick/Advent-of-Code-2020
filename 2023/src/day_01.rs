pub fn solve(input: &str) -> (u32, u64) {
    let mut sum = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        for char in line.as_bytes() {
            if char.is_ascii_digit() {
                if first.is_none() {
                    first = Some(char);
                }
                last = Some(char);
            }
        }
        sum += String::from_utf8(vec![*first.unwrap(), *last.unwrap()]).unwrap().parse::<u32>().unwrap();
    }
    (sum, 0)
}
