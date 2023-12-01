pub fn solve(input: &str) -> (u32, u32) {
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for line in input.lines() {
        let mut first_1 = None;
        let mut last_1 = None;
        let mut first_2 = None;
        let mut last_2 = None;

        for i in 0..line.len() {
            let char = line.as_bytes()[i];

            let mut digit = None;

            if char.is_ascii_digit() {
                digit = Some(char);
                if first_1.is_none() {
                    first_1 = Some(char);
                }
                last_1 = Some(char);
            }

            if line[i..].starts_with("one") {
                digit = Some(b'1');
            }
            if line[i..].starts_with("two") {
                digit = Some(b'2');
            }
            if line[i..].starts_with("three") {
                digit = Some(b'3');
            }
            if line[i..].starts_with("four") {
                digit = Some(b'4');
            }
            if line[i..].starts_with("five") {
                digit = Some(b'5');
            }
            if line[i..].starts_with("six") {
                digit = Some(b'6');
            }
            if line[i..].starts_with("seven") {
                digit = Some(b'7');
            }
            if line[i..].starts_with("eight") {
                digit = Some(b'8');
            }
            if line[i..].starts_with("nine") {
                digit = Some(b'9');
            }

            if let Some(digit) = digit {
                if first_2.is_none() {
                    first_2 = Some(digit);
                }
                last_2 = Some(digit);
            }
        }

        sum_1 += String::from_utf8(vec![first_1.unwrap(), last_1.unwrap()])
            .unwrap()
            .parse::<u32>()
            .unwrap();
        sum_2 += String::from_utf8(vec![first_2.unwrap(), last_2.unwrap()])
            .unwrap()
            .parse::<u32>()
            .unwrap();
    }

    (sum_1, sum_2)
}
