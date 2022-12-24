use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Clone, Copy)]
enum Dir4 {
    N,
    E,
    S,
    W,
}

impl Dir4 {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            Self::N => (0, -1),
            Self::E => (1, 0),
            Self::S => (0, 1),
            Self::W => (-1, 0),
        }
    }
}

fn is_wall(size: (i32, i32), pos: (i32, i32)) -> bool {
    if pos.1 < 0 || pos.1 > size.1 - 1 {
        return true;
    }
    if pos.1 == 0 {
        return pos.0 != 1;
    }
    if pos.1 == size.1 - 1 {
        return pos.0 != size.0 - 2;
    }
    return pos.0 == 0 || pos.0 == size.0 - 1;
}

fn simulate(
    size: (i32, i32),
    blizzards: &HashMap<(i32, i32), Vec<Dir4>>,
) -> HashMap<(i32, i32), Vec<Dir4>> {
    let mut new_blizzards = HashMap::new();

    for (pos, dirs) in blizzards.iter() {
        for dir in dirs.iter() {
            let vec = dir.to_vec();
            let mut next_pos = (pos.0 + vec.0, pos.1 + vec.1);

            if next_pos.0 == 0 {
                next_pos.0 = size.0 - 2;
            } else if next_pos.0 == size.0 - 1 {
                next_pos.0 = 1;
            } else if next_pos.1 == 0 {
                next_pos.1 = size.1 - 2;
            } else if next_pos.1 == size.1 - 1 {
                next_pos.1 = 1;
            }

            new_blizzards.entry(next_pos)
                .and_modify(|d: &mut Vec<Dir4>| d.push(*dir))
                .or_insert(vec![*dir]);
        }
    }

    new_blizzards
}

fn search(
    size: (i32, i32),
    start_pos: (i32, i32),
    end_pos: (i32, i32),
    minutes: usize,
    blizzards: &mut Vec<HashMap<(i32, i32), Vec<Dir4>>>,
) -> usize {
    let mut queue = VecDeque::<((i32, i32), usize)>::new();
    let mut visited = HashSet::<((i32, i32), usize)>::new();

    queue.push_back((start_pos, minutes));
    visited.insert((start_pos, minutes));

    while let Some((pos, minutes)) = queue.pop_front() {
        if pos.0 == end_pos.0 && pos.1 == end_pos.1 {
            return minutes;
        }

        while minutes + 1 >= blizzards.len() {
            blizzards.push(simulate(size, &blizzards[blizzards.len() - 1]));
        }

        for vec in [(0, -1), (1, 0), (0, 1), (-1, 0), (0, 0)] {
            let next_pos = (pos.0 + vec.0, pos.1 + vec.1);
            let vertex = (next_pos, minutes + 1);

            if is_wall(size, next_pos) { continue }
            if visited.contains(&vertex) { continue }
            if blizzards[minutes + 1].contains_key(&next_pos) { continue }

            queue.push_back(vertex);
            visited.insert(vertex);
        }
    }

    return usize::MAX;
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;
    let mut blizzards = HashMap::<(i32, i32), Vec<Dir4>>::new();

    for (y, line) in input.lines().enumerate() {
        width = line.len() as i32;
        height = (y + 1) as i32;
        for (x, b) in line.bytes().enumerate() {
            let dir = match b {
                b'^' => Dir4::N,
                b'>' => Dir4::E,
                b'v' => Dir4::S,
                b'<' => Dir4::W,
                _ => continue,
            };
            blizzards.insert((x as i32, y as i32), vec![dir]);
        }
    }

    let size = (width, height);
    let top_left = (1, 0);
    let bottom_right = (size.0 - 2, size.1 - 1);
    let mut blizzards = vec![blizzards];

    let part_1 = search(size, top_left, bottom_right, 0, &mut blizzards);
    let back = search(size, bottom_right, top_left, part_1, &mut blizzards);
    let part_2 = search(size, top_left, bottom_right, back, &mut blizzards);

    (part_1, part_2)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    let output = solve(input);
    assert_eq!(output.0, 18);
    assert_eq!(output.1, 54);
}
