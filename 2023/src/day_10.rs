use crate::common;

pub fn solve(input: &str) -> (u64, u64) {
    let input = input.as_bytes();
    let width = input.iter().position(|b| *b == b'\n').unwrap();
    let stride = width + 1;
    let height = (input.len() + 1) / stride;
    let input = unsafe { std::mem::transmute::<&[u8], &[Tile]>(input) };

    let i_to_xy = |i: usize| -> (i32, i32) {
        ((i % stride) as i32, (i / stride) as i32)
    };
    let xy_to_i = |(x, y): (i32, i32)| -> usize {
        y as usize * stride + x as usize
    };

    let start_i = input.iter().position(|t| *t == Tile::Start).unwrap();
    let start_pos = i_to_xy(start_i);

    let mut routes = Vec::new();

    for dir in Dir::ALL {
        let next_pos = common::add(start_pos, dir.to_vec());
        let back_dir = dir.opposite();
        if input[xy_to_i(next_pos)].connects(back_dir) {
            routes.push((next_pos, back_dir))
        }
    }

    assert_eq!(routes.len(), 2);

    let mut steps = 1;

    while routes[0].0 != routes[1].0 {
        for (pos, back_dir) in routes.iter_mut() {
            let next_dir = input[xy_to_i(*pos)].other_connection(*back_dir);
            common::add_assign(pos, next_dir.to_vec());
            *back_dir = next_dir.opposite();
        }

        steps += 1;
    }

    (steps, 0)
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
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
