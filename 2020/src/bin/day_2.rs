use text_io::scan;
use adventofcode2020::*;

fn main() {
    let mut part_one = 0;
    let mut part_two = 0;

    lines_from_file("input/day_2.txt", |line| {
        let low: usize;
        let high: usize;
        let letter: char;
        let password: String;
        scan!(line.bytes() => "{}-{} {}: {}", low, high, letter, password);

        let occurrences = password.chars().filter(|ch| *ch == letter).count();
        if low <= occurrences && occurrences <= high {
            part_one += 1;
        }

        let letter = Some(letter);
        let low_char = password.chars().nth(low - 1);
        let high_char = password.chars().nth(high - 1);
        part_two += ((low_char == letter) != (high_char == letter)) as i32;
    });

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
