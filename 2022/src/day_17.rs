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

pub fn solve(input: &str) -> (usize, usize) {
    // positive is up
    let mut chamber = Vec::<u8>::new();
    let mut falling_rock_idx = 0;
    let mut falling_rock = Vec::<u8>::new();
    let mut falling_rock_y;
    let mut jet = input.bytes().filter(|b| *b == b'<' || *b == b'>').cycle();

    for _ in 0..2022 {
        falling_rock.clear();
        falling_rock.extend_from_slice(ROCKS[falling_rock_idx]);

        let base_size = chamber.iter().rposition(|row| *row != 0).map(|i| i + 1).unwrap_or(0);
        falling_rock_y = base_size + 3;
        let new_size = falling_rock_y + falling_rock.len();
        chamber.resize(new_size, 0);

        loop {
            let dir = jet.next().unwrap();

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

            let mut collide = false;
            for (y, row) in falling_rock.iter().enumerate() {
                if (row & chamber[falling_rock_y + y - 1]) != 0 {
                    // hitting another rock
                    collide = true;
                    break;
                }
            }
            if !collide {
                falling_rock_y -= 1;
            } else {
                break;
            }
        }

        // coming to rest
        for (y, row) in falling_rock.iter().enumerate() {
            chamber[falling_rock_y + y] |= row;
        }

        falling_rock_idx = (falling_rock_idx + 1) % ROCKS.len();
    }

    let count = chamber.iter().rposition(|row| *row != 0).map(|i| i + 1).unwrap();

    (count, 0)
}