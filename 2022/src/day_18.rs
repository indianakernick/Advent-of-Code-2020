use std::collections::HashSet;
use text_io::scan;

type Coord = i8;
type Coord3 = (Coord, Coord, Coord);

fn touch_or_visit(
    lava: &mut HashSet<Coord3>,
    visited: &mut HashSet<Coord3>,
    minimum: Coord3,
    maximum: Coord3,
    next: Coord3,
) -> u32 {
    if lava.contains(&next) {
        1
    } else {
        flood_fill(lava, visited, minimum, maximum, next)
    }
}

fn flood_fill(
    lava: &mut HashSet<Coord3>,
    visited: &mut HashSet<Coord3>,
    minimum: Coord3,
    maximum: Coord3,
    current: Coord3,
) -> u32 {
    if !visited.insert(current) {
        return 0;
    }

    let mut count = 0;

    if current.0 > minimum.0 {
        let next = (current.0 - 1, current.1, current.2);
        count += touch_or_visit(lava, visited, minimum, maximum, next);
    }

    if current.0 < maximum.0 {
        let next = (current.0 + 1, current.1, current.2);
        count += touch_or_visit(lava, visited, minimum, maximum, next);
    }

    if current.1 > minimum.1 {
        let next = (current.0, current.1 - 1, current.2);
        count += touch_or_visit(lava, visited, minimum, maximum, next);
    }

    if current.1 < maximum.1 {
        let next = (current.0, current.1 + 1, current.2);
        count += touch_or_visit(lava, visited, minimum, maximum, next);
    }

    if current.2 > minimum.2 {
        let next = (current.0, current.1, current.2 - 1);
        count += touch_or_visit(lava, visited, minimum, maximum, next);
    }

    if current.2 < maximum.2 {
        let next = (current.0, current.1, current.2 + 1);
        count += touch_or_visit(lava, visited, minimum, maximum, next);
    }

    count
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut lava = HashSet::<Coord3>::with_capacity(input.len() / 8);

    for line in input.lines() {
        let x: Coord;
        let y: Coord;
        let z: Coord;
        scan!(line.bytes() => "{},{},{}", x, y, z);
        lava.insert((x, y, z));
    }

    let mut interior_exposed = 0;
    let mut minimum = (Coord::MAX, Coord::MAX, Coord::MAX);
    let mut maximum = (Coord::MIN, Coord::MIN, Coord::MIN);

    for pos in lava.iter() {
        if !lava.contains(&(pos.0 - 1, pos.1, pos.2)) {
            interior_exposed += 1;
        }
        if !lava.contains(&(pos.0 + 1, pos.1, pos.2)) {
            interior_exposed += 1;
        }
        if !lava.contains(&(pos.0, pos.1 - 1, pos.2)) {
            interior_exposed += 1;
        }
        if !lava.contains(&(pos.0, pos.1 + 1, pos.2)) {
            interior_exposed += 1;
        }
        if !lava.contains(&(pos.0, pos.1, pos.2 - 1)) {
            interior_exposed += 1;
        }
        if !lava.contains(&(pos.0, pos.1, pos.2 + 1)) {
            interior_exposed += 1;
        }

        minimum.0 = minimum.0.min(pos.0);
        minimum.1 = minimum.1.min(pos.1);
        minimum.2 = minimum.2.min(pos.2);
        maximum.0 = maximum.0.max(pos.0);
        maximum.1 = maximum.1.max(pos.1);
        maximum.2 = maximum.2.max(pos.2);
    }

    let minimum = (minimum.0 - 1, minimum.2 - 1, minimum.2 - 1);
    let maximum = (maximum.0 + 1, maximum.2 + 1, maximum.2 + 1);
    let mut visited = HashSet::with_capacity(lava.len());
    let exterior_exposed = flood_fill(&mut lava, &mut visited, minimum, maximum, minimum);

    (interior_exposed, exterior_exposed)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    let output = solve(input);
    assert_eq!(output.0, 64);
    assert_eq!(output.1, 58);
}
