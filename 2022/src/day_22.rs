use std::collections::HashMap;

enum Tile {
    Walkable,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    Walk(i32),
    Left,
    Right,
}

#[derive(Debug)]
#[repr(u8)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

pub fn solve(input: &str) -> (i32, usize) {
    let mut tiles = HashMap::<(i32, i32), Tile>::new();
    let mut instructions = Vec::<Instruction>::new();
    let mut read_instructions = false;
    let mut position = (0i32, 0i32);
    let mut direction = Direction::Right;

    for (y, line) in input.lines().enumerate() {
        if read_instructions {
            let bytes = line.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i].is_ascii_digit() {
                    let end = bytes[i..].iter().position(|b| !b.is_ascii_digit()).unwrap_or(bytes.len() - i);
                    let num = line[i..i + end].parse().unwrap();
                    i += end - 1;
                    instructions.push(Instruction::Walk(num));
                } else if bytes[i] == b'L' {
                    instructions.push(Instruction::Left);
                } else if bytes[i] == b'R' {
                    instructions.push(Instruction::Right);
                }
                i += 1;
            }
            break;
        }

        if line.is_empty() {
            read_instructions = true;
            continue;
        }

        for (x, b) in line.bytes().enumerate() {
            let tile = match b {
                b' ' => continue,
                b'.' => Tile::Walkable,
                b'#' => Tile::Wall,
                _ => panic!()
            };
            if tiles.is_empty() {
                position = (x as i32, 0);
            }
            tiles.insert((x as i32, y as i32), tile);
        }
    }

    for instr in instructions.iter() {
        match instr {
            Instruction::Walk(dist) => {
                let vec = direction.to_vec();
                for _ in 0..*dist {
                    let next_pos = (position.0 + vec.0, position.1 + vec.1);
                    match tiles.get(&next_pos) {
                        Some(Tile::Walkable) => {
                            position = next_pos;
                        }
                        Some(Tile::Wall) => {
                            break;
                        }
                        None => {
                            let mut pos = position;
                            while tiles.get(&pos).is_some() {
                                pos = (pos.0 - vec.0, pos.1 - vec.1);
                            }
                            pos = (pos.0 + vec.0, pos.1 + vec.1);
                            match tiles.get(&pos).unwrap() {
                                Tile::Walkable => { position = pos; }
                                Tile::Wall => break,
                            }
                        }
                    }
                }
            }
            Instruction::Left => {
                direction = unsafe { std::mem::transmute(((direction as u8) + 4 - 1) % 4) };
            }
            Instruction::Right => {
                direction = unsafe { std::mem::transmute(((direction as u8) + 1) % 4) };
            }
        }
    }

    let password = 1000 * (position.1 + 1) + 4 * (position.0 + 1) + direction as i32;

    (password, 0)
}
