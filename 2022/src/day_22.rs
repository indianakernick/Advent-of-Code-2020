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
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Up => (0, -1),
        }
    }

    pub fn rotate_cw(&self) -> Self {
        unsafe { std::mem::transmute((*self as u8 + 1) % 4) }
    }

    pub fn rotate_ccw(&self) -> Self {
        unsafe { std::mem::transmute((*self as u8 + 4 - 1) % 4) }
    }
}

fn get_password(position: (i32, i32), direction: Dir) -> i32 {
    1000 * (position.1 + 1) + 4 * (position.0 + 1) + direction as i32
}

fn simulate<F>(
    tiles: &HashMap::<(i32, i32), Tile>,
    instructions: &[Instruction],
    mut position: (i32, i32),
    mut direction: Dir,
    edge: F
) -> ((i32, i32), Dir)
    where F: Fn((i32, i32), Dir) -> ((i32, i32), Dir)
{
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
                        Some(Tile::Wall) => break,
                        None => {
                            let (next_pos, next_dir) = edge(position, direction);
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
                direction = direction.rotate_ccw();
            }
            Instruction::Right => {
                direction = direction.rotate_cw();
            }
        }
    }

    (position, direction)
}

fn flat_edge(position: (i32, i32), direction: Dir) -> ((i32, i32), Dir) {
    let super_tile = (position.0 / 50, position.1 / 50);
    let pos = match (super_tile, direction) {
        ((1, 0), Dir::Left) => (149, position.1),
        ((1, 0), Dir::Up) => (position.0, 149),
        ((2, 0), Dir::Right) => (50, position.1),
        ((2, 0), Dir::Down) => (position.0, 0),
        ((2, 0), Dir::Up) => (position.0, 49),
        ((1, 1), Dir::Right) => (50, position.1),
        ((1, 1), Dir::Left) => (99, position.1),
        ((0, 2), Dir::Left) => (99, position.1),
        ((0, 2), Dir::Up) => (position.0, 199),
        ((1, 2), Dir::Right) => (0, position.1),
        ((1, 2), Dir::Down) => (position.0, 0),
        ((0, 3), Dir::Right) => (0, position.1),
        ((0, 3), Dir::Down) => (position.0, 100),
        ((0, 3), Dir::Left) => (49, position.1),
        _ => panic!(),
    };
    (pos, direction)
}

fn cube_edge(position: (i32, i32), direction: Dir) -> ((i32, i32), Dir) {
    let super_tile = (position.0 / 50, position.1 / 50);
    let in_tile = (position.0 % 50, position.1 % 50);
    match (super_tile, direction) {
        ((1, 0), Dir::Left) => ((0, 100 + 49 - in_tile.1), Dir::Right),
        ((1, 0), Dir::Up) => ((0, 150 + in_tile.0), Dir::Right),
        ((2, 0), Dir::Right) => ((99, 100 + 49 - in_tile.1), Dir::Left),
        ((2, 0), Dir::Down) => ((99, 50 + in_tile.0), Dir::Left),
        ((2, 0), Dir::Up) => ((in_tile.0, 199), Dir::Up),
        ((1, 1), Dir::Right) => ((100 + in_tile.1, 49), Dir::Up),
        ((1, 1), Dir::Left) => ((in_tile.1, 100), Dir::Down),
        ((0, 2), Dir::Left) => ((50, 49 - in_tile.1), Dir::Right),
        ((0, 2), Dir::Up) => ((50, 50 + in_tile.0), Dir::Right),
        ((1, 2), Dir::Right) => ((149, 49 - in_tile.1), Dir::Left),
        ((1, 2), Dir::Down) => ((49, 150 + in_tile.0), Dir::Left),
        ((0, 3), Dir::Right) => ((50 + in_tile.1, 149), Dir::Up),
        ((0, 3), Dir::Down) => ((100 + in_tile.0, 0), Dir::Down),
        ((0, 3), Dir::Left) => ((50 + in_tile.1, 0), Dir::Down),
        _ => panic!(),
    }
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut tiles = HashMap::<(i32, i32), Tile>::new();
    let mut instructions = Vec::<Instruction>::new();
    let mut read_instructions = false;
    let mut position = (0, 0);
    let direction = Dir::Right;

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

    let (position_1, direction_1) = simulate(&tiles, &instructions, position, direction, flat_edge);
    let password_1 = get_password(position_1, direction_1);
    let (position_2, direction_2) = simulate(&tiles, &instructions, position, direction, cube_edge);
    let password_2 = get_password(position_2, direction_2);

    (password_1, password_2)
}

// No test because the flat_edge and cube_edge functions make some assumptions
// about the input that don't hold for the example input.
