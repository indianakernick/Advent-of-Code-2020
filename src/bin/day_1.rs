use adventofcode2020::*;

fn main() {
    let mut numbers = Vec::new();
    lines_from_file("input/day_1.txt", |line| {
        let num: i32 = read!("{}", line.bytes());
        numbers.push(num);
    });

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("Part one: {}", numbers[i] * numbers[j]);
                break;
            }
        }
    }

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            for k in j..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("Part two: {}", numbers[i] * numbers[j] * numbers[k]);
                    break;
                }
            }
        }
    }
}
