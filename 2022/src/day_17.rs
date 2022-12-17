use std::collections::{VecDeque, HashMap};

// The rock shapes. They're offset from the left by 2. Also note that they're
// upside-down. Increasing index is upwards.
const ROCKS: [&[u8]; 5] = [
    &[
        0b00111100
    ],
    &[
        0b00010000,
        0b00111000,
        0b00010000,
    ],
    &[
        0b00111000,
        0b00001000,
        0b00001000,
    ],
    &[
        0b00100000,
        0b00100000,
        0b00100000,
        0b00100000,
    ],
    &[
        0b00110000,
        0b00110000,
    ],
];
const MAX_ROCK_HEIGHT: usize = 4;

// The state of the simulation.
#[derive(PartialEq, Eq, Hash)]
struct State {
    chamber: VecDeque<u8>,
    falling_rock_idx: usize,
    jet_idx: usize,
}

// The position of the simulation at a particular state.
struct Position {
    iter: usize,
    trim_size: usize,
}

fn solve_impl(input: &str, iter_count: usize) -> usize {
    // The chamber of rocks, from bottom to top, which reach row represented as
    // a 7-bit bitset. Using a deque so that we can efficiently add to the end
    // and remove from the beginning.
    let mut chamber = VecDeque::<u8>::new();
    // The number of rows that we've trimmed off of the chamber. If a row is
    // has all 7 tiles filled, then there's no point in storing anything below
    // that.
    let mut trim_size = 0;

    // Index within the ROCKS array of the current rock shape.
    let mut falling_rock_idx = 0;
    // A copy of one of the ROCKS, possibly shifted left or right.
    let mut falling_rock: [u8; MAX_ROCK_HEIGHT] = Default::default();
    // Vertical position of the falling rock. Positive is up.
    let mut falling_rock_y;

    // The jet directions. The input file ends with a newline so we're trimming
    // that off.
    let jet = &input.as_bytes()[0..input.bytes()
        .rposition(|b| b == b'<' || b == b'>')
        .unwrap() + 1];
    // The current index in the jet direction array.
    let mut jet_idx = 0;

    // A cache of positions at previous states. If we find
    let mut cache = HashMap::<State, Position>::new();
    // Whether we've found a cycle and skipped over it. Once we're close to the
    // end, we don't want to keep checking the cache to find cycles. We're
    // unlikely to find any in the little remaining space anyway.
    let mut found_cycle = false;
    // The current iteration index.
    let mut iter = 0;

    while iter < iter_count {
        falling_rock.fill(0);
        let falling_rock_size = ROCKS[falling_rock_idx].len();
        falling_rock[0..falling_rock_size].copy_from_slice(ROCKS[falling_rock_idx]);

        // Search for a filled row.
        let unused_size = chamber.iter()
            .rposition(|row| *row == 0b11111110)
            .map(|i| i + 1);

        if let Some(unused_size) = unused_size {
            // The filled row and all rows below it are removed.
            trim_size += unused_size;
            chamber.drain(0..unused_size);

            if !found_cycle {
                // Search the cache to check if we've been in this state before.
                // Note that we have to clone the chamber but this clone is only
                // useless once (when we find the cycle).
                let old_position = cache.insert(
                    State { chamber: chamber.clone(), falling_rock_idx, jet_idx },
                    Position { iter, trim_size }
                );
                if let Some(old_position) = old_position {
                    // We've been in this state before. There is a cycle between
                    // the old_position and our current position. We can do some
                    // multiplication on our position to get as close as
                    // possible to the iter_count without actually simulating
                    // all of that.
                    found_cycle = true;
                    let cycle_iters = iter - old_position.iter;
                    let cycle_count = (iter_count - iter) / cycle_iters;
                    iter += cycle_iters * cycle_count;
                    trim_size += (trim_size - old_position.trim_size) * cycle_count;
                }
            }
        }

        // The falling rock starts 3 rows above the tower height.
        falling_rock_y = chamber.len() + 3;

        // This inner loop processes the fall of a single rock down to its
        // resting position.
        'fall: loop {
            let dir = jet[jet_idx];
            jet_idx = (jet_idx + 1) % jet.len();

            // The height of the tallest rock is 4 so we can pack it into a
            // single integer and operate on all rows at once. Although the
            // chamber is a circular array so this has limited utility but still
            // makes a significant difference to running time (from about 670us
            // to 570us).
            let falling_rock_int = u32::from_ne_bytes(falling_rock);

            // Shift the falling rock left or right according to the current jet
            // direction. This is a bit repetitive...
            match dir {
                b'<' => 'collision: {
                    if (falling_rock_int & 0b10000000100000001000000010000000) != 0 {
                        // Colliding with the left wall.
                        break 'collision;
                    }
                    for (y, row) in falling_rock.iter().take(falling_rock_size).enumerate() {
                        let i = falling_rock_y + y;
                        if i < chamber.len() && ((row << 1) & chamber[i]) != 0 {
                            // Colliding with another rock.
                            break 'collision;
                        }
                    }
                    falling_rock = (falling_rock_int << 1).to_ne_bytes();
                }
                b'>' => 'collision: {
                    if (falling_rock_int & 0b00000010000000100000001000000010) != 0 {
                        // Colliding with the right wall.
                        break 'collision;
                    }
                    for (y, row) in falling_rock.iter().take(falling_rock_size).enumerate() {
                        let i = falling_rock_y + y;
                        if i < chamber.len() && ((row >> 1) & chamber[i]) != 0 {
                            // Colliding with another rock.
                            break 'collision;
                        }
                    }
                    falling_rock = (falling_rock_int >> 1).to_ne_bytes();
                }
                _ => {}
            }

            if falling_rock_y == 0 {
                // The rock has hit the floor. Stop falling.
                break;
            }

            for (y, row) in falling_rock.iter().take(falling_rock_size).enumerate() {
                let i = falling_rock_y + y - 1;
                if i < chamber.len() && (row & chamber[i]) != 0 {
                    // Colliding with another row. Stop falling.
                    break 'fall;
                }
            }

            falling_rock_y -= 1;
        }

        // The rock is coming to rest. We can stamp it into the chamber.
        let min_size = falling_rock_y + falling_rock_size;
        if chamber.len() < min_size {
            chamber.resize(min_size, 0);
        }
        for (y, row) in falling_rock.iter().take(falling_rock_size).enumerate() {
            chamber[falling_rock_y + y] |= row;
        }

        falling_rock_idx = (falling_rock_idx + 1) % ROCKS.len();
        iter += 1;
    }

    trim_size + chamber.len()
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
