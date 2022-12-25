const MAX_LENGTH: usize = 27; // log_5(2^63)

const fn calc_powers() -> [i64; MAX_LENGTH] {
    let mut powers = [0; MAX_LENGTH];
    let mut i = 0;
    while i < MAX_LENGTH {
        powers[i] = (5i64).pow(i as u32);
        i += 1;
    }
    powers
}

const POWERS: [i64; MAX_LENGTH] = calc_powers();

pub fn solve(input: &str) -> (String, usize) {
    let mut sum = 0;

    for line in input.lines() {
        for (place, b) in line.bytes().rev().enumerate() {
            sum += POWERS[place] * match b {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => panic!("Invalid input"),
            };
        }
    }

    let mut result = String::with_capacity(MAX_LENGTH);
    let mut started = false;

    for i in (0..MAX_LENGTH).rev() {
        let power = POWERS[i];
        let mut min = (i64::MAX, 0, '0');

        for digit in [2, 1, 0, -1, -2] {
            let place_value = power * digit;
            let abs = (sum - place_value).abs();
            if abs < min.0 {
                min = (abs, place_value, match digit {
                    2 => '2',
                    1 => '1',
                    0 => '0',
                    -1 => '-',
                    -2 => '=',
                    _ => unreachable!(),
                });
            }
        }

        if min.1 != 0 {
            started = true;
        } else if !started {
            continue;
        }

        sum -= min.1;
        result.push(min.2);
    }

    (result, 0)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    let output = solve(input);
    assert_eq!(output.0, "2=-1=0");
}
