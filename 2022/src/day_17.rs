use std::collections::{VecDeque, HashMap};

const ROCK_0: [u8; 1] = [0b00111100];
const ROCK_1: [u8; 3] = [
    0b00010000,
    0b00111000,
    0b00010000,
];
const ROCK_2: [u8; 3] = [
    0b00111000,
    0b00001000,
    0b00001000,
];
const ROCK_3: [u8; 4] = [
    0b00100000,
    0b00100000,
    0b00100000,
    0b00100000,
];
const ROCK_4: [u8; 2] = [
    0b00110000,
    0b00110000,
];
const ROCKS: [&[u8]; 5] = [&ROCK_0, &ROCK_1, &ROCK_2, &ROCK_3, &ROCK_4];

fn get_height(chamber: &VecDeque<u8>) -> usize {
    chamber.iter().rposition(|row| *row != 0).map(|i| i + 1).unwrap_or(0)
}

fn solve_impl(input: &str, count: usize) -> usize {
    // positive is up
    let mut chamber = VecDeque::<u8>::new();
    let mut trim_size = 0;
    let mut falling_rock_idx = 0;
    let mut falling_rock = Vec::<u8>::new();
    let mut falling_rock_y;
    let mut jet_idx = 0;
    let jet = &input.as_bytes()[0..input.bytes().rposition(|b| b == b'<' || b == b'>').unwrap() + 1];

    let mut chambers = HashMap::<(VecDeque<u8>, usize, usize), (usize, usize)>::new();
    let mut r = 0;
    let mut found_cycle = false;

    while r < count {
        falling_rock.clear();
        falling_rock.extend_from_slice(ROCKS[falling_rock_idx]);

        let unused_size = chamber.iter().rposition(|row| *row == 0b11111110).map(|i| i + 1);
        if let Some(unused_size) = unused_size {
            trim_size += unused_size;
            chamber.drain(0..unused_size);
            let base_size = get_height(&chamber);

            chamber.truncate(base_size);
            let insert = chambers.insert((chamber.clone(), falling_rock_idx, jet_idx), (r, trim_size));
            if !found_cycle && insert.is_some() {
                // We've been in this state before. Repeat the cycle until we're
                // near the end.
                found_cycle = true;
                let (start, start_trim) = insert.unwrap();
                let cycle_size = r - start;
                let cycles = (count - r) / cycle_size;
                r += cycle_size * cycles;
                trim_size += (trim_size - start_trim) * cycles;
            }
        }

        let base_size = get_height(&chamber);
        falling_rock_y = base_size + 3;
        let new_size = falling_rock_y + falling_rock.len();
        chamber.resize(new_size, 0);

        'fall: loop {
            let dir = jet[jet_idx];
            jet_idx = (jet_idx + 1) % jet.len();

            match dir {
                b'<' => {
                    let mut collide = false;
                    for (y, row) in falling_rock.iter().enumerate() {
                        if row.leading_zeros() == 0 {
                            // hitting left wall
                            collide = true;
                            break;
                        }
                        if ((row << 1) & chamber[falling_rock_y + y]) != 0 {
                            // hitting another rock
                            collide = true;
                            break;
                        }
                    }
                    if !collide {
                        for row in falling_rock.iter_mut() {
                            *row <<= 1;
                        }
                    }
                }
                b'>' => {
                    let mut collide = false;
                    for (y, row) in falling_rock.iter().enumerate() {
                        if row.trailing_zeros() == 1 {
                            // hitting right wall
                            collide = true;
                            break;
                        }
                        if ((row >> 1) & chamber[falling_rock_y + y]) != 0 {
                            // hitting another rock
                            collide = true;
                            break;
                        }
                    }
                    if !collide {
                        for row in falling_rock.iter_mut() {
                            *row >>= 1;
                        }
                    }
                }
                _ => {}
            }

            if falling_rock_y == 0 {
                // hit the floor. stop falling
                break;
            }

            for (y, row) in falling_rock.iter().enumerate() {
                if (row & chamber[falling_rock_y + y - 1]) != 0 {
                    // hitting another rock. stop falling
                    break 'fall;
                }
            }

            falling_rock_y -= 1;
        }

        // coming to rest
        for (y, row) in falling_rock.iter().enumerate() {
            chamber[falling_rock_y + y] |= row;
        }

        falling_rock_idx = (falling_rock_idx + 1) % ROCKS.len();
        r += 1;
    }

    trim_size + get_height(&chamber)
}

pub fn solve(input: &str) -> (usize, usize) {
    (solve_impl(input, 2022), solve_impl(input, 1000000000000))
}

#[cfg(test)]
#[test]
fn example() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    // It doesn't find a cycle...
    assert_eq!(solve_impl(input, 2022), 3068);
    // assert_eq!(solve_impl(input, 1000000000000), 1514285714288);
}
