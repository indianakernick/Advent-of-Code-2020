use adventofcode2020::*;

fn parse_input() -> Vec<i64> {
    line_iter_from_file("input/day_9.txt")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn first_bad_number(numbers: &Vec<i64>) -> i64 {
    for i in 25..numbers.len() {
        let mut found = false;
        for first in (i - 25)..i {
            for second in first..i {
                if numbers[first] != numbers[second] && numbers[first] + numbers[second] == numbers[i] {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return numbers[i];
        }
    }
    panic!();
}

fn contiguous_sum(numbers: &Vec<i64>, first_bad: i64) -> i64 {
    for end in 2..numbers.len() {
        for start in 0..(end - 2) {
            let mut sum = 0;
            let mut min = i64::MAX;
            let mut max = i64::MIN;
            for num in start..end {
                sum += numbers[num];
                min = min.min(numbers[num]);
                max = max.max(numbers[num]);
            }
            if sum == first_bad {
                return min + max;
            }
        }
    }
    panic!();
}

fn main() {
    let numbers = parse_input();
    let first_bad = first_bad_number(&numbers);
    println!("Part one: {}", first_bad);
    println!("Part two: {}", contiguous_sum(&numbers, first_bad));
}
