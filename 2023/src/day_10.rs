use crate::common::{self, Grid, Dir};

pub fn solve(input: &str) -> (i32, i32) {
    let grid = Grid::<Tile>::from_input(input);
    let start_pos = grid.pos_of(Tile::Start).unwrap();

    let mut routes = [((0, 0), Dir::N); 2];
    let mut routes_len = 0;

    for dir in Dir::ALL {
        let next_pos = common::add(start_pos, dir.to_vec());
        if !grid.valid(next_pos) {
            continue;
        }
        let back_dir = dir.opposite();
        if grid.get(next_pos).connects(back_dir) {
            assert!(routes_len < 2, "Invalid input");
            routes[routes_len] = (next_pos, back_dir);
            routes_len += 1;
        }
    }

    assert_eq!(routes_len, 2, "Invalid input");

    let mut borders = (Vec::new(), Vec::new());
    let mut steps = 1;

    borders.0.push(start_pos);

    while routes[0].0 != routes[1].0 {
        fn follow(
            grid: &Grid<Tile>,
            (pos, back_dir): &mut ((i32, i32), Dir),
            borders: &mut Vec<(i32, i32)>
        ) {
            borders.push(*pos);
            let next_dir = grid.get(*pos).other_connection(*back_dir);
            common::add_assign(pos, next_dir.to_vec());
            *back_dir = next_dir.opposite();
        }

        follow(&grid, &mut routes[0], &mut borders.0);
        follow(&grid, &mut routes[1], &mut borders.1);

        steps += 1;
    }

    borders.0.push(routes[0].0);
    borders.0.extend(borders.1.iter().rev());
    borders.0.push(borders.0[0]);

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let interior_count = borders.0
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
    fn connects(self, dir: Dir) -> bool {
        match (self, dir) {
            (Self::NS, Dir::N) => true,
            (Self::NS, Dir::S) => true,
            (Self::EW, Dir::E) => true,
            (Self::EW, Dir::W) => true,
            (Self::NE, Dir::N) => true,
            (Self::NE, Dir::E) => true,
            (Self::NW, Dir::N) => true,
            (Self::NW, Dir::W) => true,
            (Self::SW, Dir::S) => true,
            (Self::SW, Dir::W) => true,
            (Self::SE, Dir::E) => true,
            (Self::SE, Dir::S) => true,
            _ => false,
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
