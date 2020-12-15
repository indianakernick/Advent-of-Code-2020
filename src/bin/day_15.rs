fn find_spoken_number(start: &[u32], last_turn: u32) -> u32 {
    let mut numbers = Vec::<(u32, u32)>::new();
    numbers.resize(last_turn as usize, (u32::MAX, u32::MAX));
    for i in 0..start.len() {
        let turn = i as u32;
        numbers[start[i] as usize] = (turn, turn);
    }
    let mut last = start[start.len() - 1];

    for t in (start.len() as u32)..last_turn {
        let pair = &mut numbers[last as usize];
        last = pair.1 - pair.0;
        let pair = &mut numbers[last as usize];
        if pair.0 == u32::MAX && pair.1 == u32::MAX {
            *pair = (t, t);
        } else {
            pair.0 = pair.1;
            pair.1 = t;
        }
    }

    last
}

fn main() {
    let start = [9, 12, 1, 4, 17, 0, 18];
    println!("Part one: {}", find_spoken_number(&start, 2020));
    println!("Part two: {}", find_spoken_number(&start, 30000000));
}
