pub fn solve(input: &str) -> (String, usize) {
    let mut powers = Vec::with_capacity(25);

    for i in 0..25 {
        powers.push((5i64).pow(i));
    }

    let mut sum = 0;

    for line in input.lines() {
        for (place, b) in line.bytes().rev().enumerate() {
            sum += powers[place] * match b {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => panic!(),
            };
        }
    }

    let mut result = String::new();

    for i in (0..25).rev() {
        let place_value = powers[i];
        let mut min = (i64::MAX, 0, '0');
        if (sum - place_value * 2).abs() < min.0 {
            min = ((sum - place_value * 2).abs(), place_value * 2, '2');
        }
        if (sum - place_value * 1).abs() < min.0 {
            min = ((sum - place_value * 1).abs(), place_value * 1, '1');
        }
        if (sum - place_value * 0).abs() < min.0 {
            min = ((sum - place_value * 0).abs(), place_value * 0, '0');
        }
        if (sum - place_value * -1).abs() < min.0 {
            min = ((sum - place_value * -1).abs(), place_value * -1, '-');
        }
        if (sum - place_value * -2).abs() < min.0 {
            min = ((sum - place_value * -2).abs(), place_value * -2, '=');
        }
        sum -= min.1;
        result.push(min.2);
    }

    (result.trim_start_matches('0').to_owned(), 0)
}
