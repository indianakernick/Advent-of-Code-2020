fn find_spoken_number(start: &[u32], last_turn: u32) -> u32 {
    let mut numbers = vec![(0, 0); last_turn as usize];
    for i in 0..start.len() {
        numbers[start[i] as usize] = (0, i as u32 + 1);
    }
    let mut last = start[start.len() - 1];

    for t in (start.len() as u32 + 1)..(last_turn + 1) {
        let pair = &numbers[last as usize];
        last = if pair.0 == 0 { 0 } else { pair.1 - pair.0 };
        let pair = &mut numbers[last as usize];
        pair.0 = pair.1;
        pair.1 = t;
    }

    last
}

fn main() {
    let start = [9, 12, 1, 4, 17, 0, 18];
    println!("Part one: {}", find_spoken_number(&start, 2020));
    println!("Part two: {}", find_spoken_number(&start, 30000000));
}
