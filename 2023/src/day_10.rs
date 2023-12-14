use crate::common;

pub fn solve(input: &str) -> (i32, i32) {
    let input = input.as_bytes();
    let width = input.iter().position(|b| *b == b'\n').unwrap();
    let stride = width + 1;
    let height = (input.len() + 1) / stride;
    let input = unsafe { std::mem::transmute::<&[u8], &[Tile]>(input) };

    let i_to_xy = |i: usize| -> (i32, i32) {
        ((i % stride) as i32, (i / stride) as i32)
    };
    // We don't really need this.
    // Should just use the index. Add stride to go down. Sub stride to go up.
    let xy_to_i = |(x, y): (i32, i32)| -> usize {
        y as usize * stride + x as usize
    };
    let valid = |(x, y): (i32, i32)| -> bool {
        0 <= x && x < width as i32 && 0 <= y && y < height as i32
    };

    let start_i = input.iter().position(|t| *t == Tile::Start).unwrap();
    let start_pos = i_to_xy(start_i);

    let mut routes = Vec::new();

    for dir in Dir::ALL {
        let next_pos = common::add(start_pos, dir.to_vec());
        if !valid(next_pos) {
            continue;
        }
        let back_dir = dir.opposite();
        if input[xy_to_i(next_pos)].connects(back_dir) {
            routes.push((next_pos, back_dir));
        }
    }

    assert_eq!(routes.len(), 2);

    let mut borders_1 = Vec::new();
    let mut borders_2 = Vec::new();
    let mut steps = 1;

    borders_1.push(start_pos);

    while routes[0].0 != routes[1].0 {
        for i in 0..2 {
            let (pos, back_dir) = &mut routes[i];
            if i == 0 { &mut borders_1 } else { &mut borders_2 }.push(*pos);
            let next_dir = input[xy_to_i(*pos)].other_connection(*back_dir);
            common::add_assign(pos, next_dir.to_vec());
            *back_dir = next_dir.opposite();
        }

        steps += 1;
    }

    borders_1.push(routes[0].0);
    borders_1.extend(borders_2.iter().rev());
    borders_1.push(borders_1[0]);

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let interior_count = borders_1
        .windows(2)
        .map(|pair| pair[0].0 * pair[1].1 - pair[1].0 * pair[0].1)
        .sum::<i32>() / 2;

    // https://en.wikipedia.org/wiki/Pick's_theorem
    // Note that `steps` is half of the number of border points.
    let area = interior_count.abs() - steps + 1;

    (steps, area)
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum Tile {
    NS = b'|',
    EW = b'-',
    NE = b'L',
    NW = b'J',
    SW = b'7',
    SE = b'F',
    Ground = b'.',
    Start = b'S',
}

impl Tile {
    fn connects_north(self) -> bool {
        match self {
            Self::NS | Self::NE | Self::NW => true,
            _ => false,
        }
    }

    fn connects_east(self) -> bool {
        match self {
            Self::EW | Self::NE | Self::SE => true,
            _ => false,
        }
    }

    fn connects_south(self) -> bool {
        match self {
            Self::NS | Self::SW | Self::SE => true,
            _ => false,
        }
    }

    fn connects_west(self) -> bool {
        match self {
            Self::EW | Self::NW | Self::SW => true,
            _ => false,
        }
    }

    fn connects(self, dir: Dir) -> bool {
        match dir {
            Dir::N => self.connects_north(),
            Dir::E => self.connects_east(),
            Dir::S => self.connects_south(),
            Dir::W => self.connects_west(),
        }
    }

    fn other_connection(self, dir: Dir) -> Dir {
        match (self, dir) {
            (Self::NS, Dir::N) => Dir::S,
            (Self::NS, Dir::S) => Dir::N,
            (Self::EW, Dir::E) => Dir::W,
            (Self::EW, Dir::W) => Dir::E,
            (Self::NE, Dir::N) => Dir::E,
            (Self::NE, Dir::E) => Dir::N,
            (Self::NW, Dir::N) => Dir::W,
            (Self::NW, Dir::W) => Dir::N,
            (Self::SW, Dir::S) => Dir::W,
            (Self::SW, Dir::W) => Dir::S,
            (Self::SE, Dir::E) => Dir::S,
            (Self::SE, Dir::S) => Dir::E,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    const ALL: [Self; 4] = [
        Self::N,
        Self::E,
        Self::S,
        Self::W,
    ];

    fn opposite(self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }

    fn to_vec(self) -> (i32, i32) {
        match self {
            Self::N => (0, -1),
            Self::E => (1, 0),
            Self::S => (0, 1),
            Self::W => (-1, 0),
        }
    }
}

#[cfg(test)]
#[test]
fn example_1() {
    let input =
"-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    let output = solve(input);
    assert_eq!(output.0, 4);
}

#[cfg(test)]
#[test]
fn example_2() {
    let input =
"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let output = solve(input);
    assert_eq!(output.0, 8);
}

#[cfg(test)]
#[test]
fn example_3() {
    let input =
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    let output = solve(input);
    assert_eq!(output.1, 4);
}

#[cfg(test)]
#[test]
fn example_4() {
    let input =
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    let output = solve(input);
    assert_eq!(output.1, 8);
}

#[cfg(test)]
#[test]
fn example_5() {
    let input =
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    let output = solve(input);
    assert_eq!(output.1, 10);
}
