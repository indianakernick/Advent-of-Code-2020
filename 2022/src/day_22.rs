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

#[derive(Debug, Clone, Copy)]
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

fn get_password(position: (i32, i32), direction: Direction) -> i32 {
    1000 * (position.1 + 1) + 4 * (position.0 + 1) + direction as i32
}

fn simulate_1(
    tiles: &HashMap::<(i32, i32), Tile>,
    instructions: &[Instruction],
    mut position: (i32, i32),
    mut direction: Direction,
) -> ((i32, i32), Direction) {
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

    (position, direction)
}

fn simulate_2(
    tiles: &HashMap::<(i32, i32), Tile>,
    instructions: &[Instruction],
    mut position: (i32, i32),
    mut direction: Direction,
) -> ((i32, i32), Direction) {
    for instr in instructions.iter() {
        match instr {
            Instruction::Walk(dist) => {
                for _ in 0..*dist {
                    let vec = direction.to_vec();
                    let next_pos = (position.0 + vec.0, position.1 + vec.1);
                    match tiles.get(&next_pos) {
                        Some(Tile::Walkable) => {
                            position = next_pos;
                        }
                        Some(Tile::Wall) => {
                            break;
                        }
                        None => {
                            let super_tile = (position.0 / 50, position.1 / 50);
                            let in_tile = (position.0 % 50, position.1 % 50);
                            let (next_pos, next_dir) = match (super_tile, direction) {
                                ((1, 0), Direction::Left) => {
                                    ((0, 100 + 49 - in_tile.1), Direction::Right)
                                },
                                ((1, 0), Direction::Up) => {
                                    ((0, 150 + in_tile.0), Direction::Right)
                                },
                                ((2, 0), Direction::Right) => {
                                    ((99, 100 + 49 - in_tile.1), Direction::Left)
                                }
                                ((2, 0), Direction::Down) => {
                                    ((99, 50 + in_tile.0), Direction::Left)
                                }
                                ((2, 0), Direction::Up) => {
                                    ((in_tile.0, 199), Direction::Up)
                                },
                                ((1, 1), Direction::Right) => {
                                    ((100 + in_tile.1, 49), Direction::Up)
                                }
                                ((1, 1), Direction::Left) => {
                                    ((in_tile.1, 100), Direction::Down)
                                },
                                ((0, 2), Direction::Left) => {
                                    ((50, 49 - in_tile.1), Direction::Right)
                                }
                                ((0, 2), Direction::Up) => {
                                    ((50, 50 + in_tile.0), Direction::Right)
                                },
                                ((1, 2), Direction::Right) => {
                                    ((149, 49 - in_tile.1), Direction::Left)
                                },
                                ((1, 2), Direction::Down) => {
                                    ((49, 150 + in_tile.0), Direction::Left)
                                },
                                ((0, 3), Direction::Right) => {
                                    ((50 + in_tile.1, 149), Direction::Up)
                                },
                                ((0, 3), Direction::Down) => {
                                    ((100 + in_tile.0, 0), Direction::Down)
                                }
                                ((0, 3), Direction::Left) => {
                                    ((50 + in_tile.1, 0), Direction::Down)
                                }
                                _ => panic!(),
                            };
                            match tiles.get(&next_pos).unwrap() {
                                Tile::Walkable => {
                                    position = next_pos;
                                    direction = next_dir;
                                }
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

    (position, direction)
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut tiles = HashMap::<(i32, i32), Tile>::new();
    let mut instructions = Vec::<Instruction>::new();
    let mut read_instructions = false;
    let mut position = (0, 0);
    let direction = Direction::Right;

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

    let (position_1, direction_1) = simulate_1(&tiles, &instructions, position, direction);
    let password_1 = get_password(position_1, direction_1);
    let (position_2, direction_2) = simulate_2(&tiles, &instructions, position, direction);
    let password_2 = get_password(position_2, direction_2);

    (password_1, password_2)
}
