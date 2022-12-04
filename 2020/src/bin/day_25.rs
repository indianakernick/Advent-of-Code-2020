fn step_forward(value: &mut u64, subject: u64) {
    *value *= subject;
    *value %= 20201227;
}

fn find_loop_size(pk: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;
    while value != pk {
        step_forward(&mut value, 7);
        loop_size += 1;
    }
    loop_size
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        step_forward(&mut value, subject);
    }
    value
}

fn main() {
    let card_pk = 8335663; // 7 * a
    let door_pk = 8614349; // 7 * b
    let card_loop_size = find_loop_size(card_pk); // a
    let door_loop_size = find_loop_size(door_pk); // b
    let _key = transform(door_pk, card_loop_size); // 7 * b * a
    let key = transform(card_pk, door_loop_size); // 7 * a * b
    println!("Part one: {}", key);
}
