use std::collections::{HashMap, HashSet};
use text_io::scan;

fn flood_fill(
    lava: &mut HashMap<(i32, i32, i32), u8>,
    visited: &mut HashSet<(i32, i32, i32)>,
    minimum: (i32, i32, i32),
    maximum: (i32, i32, i32),
    current: (i32, i32, i32),
) {
    if !visited.insert(current) {
        return;
    }

    if current.0 > minimum.0 {
        let next = (current.0 - 1, current.1, current.2);
        if let Some(bits) = lava.get_mut(&next) {
            *bits |= 1 << 0;
        } else {
            flood_fill(lava, visited, minimum, maximum, next);
        }
    }

    if current.0 < maximum.0 {
        let next = (current.0 + 1, current.1, current.2);
        if let Some(bits) = lava.get_mut(&next) {
            *bits |= 1 << 1;
        } else {
            flood_fill(lava, visited, minimum, maximum, next);
        }
    }

    if current.1 > minimum.1 {
        let next = (current.0, current.1 - 1, current.2);
        if let Some(bits) = lava.get_mut(&next) {
            *bits |= 1 << 2;
        } else {
            flood_fill(lava, visited, minimum, maximum, next);
        }
    }

    if current.1 < maximum.1 {
        let next = (current.0, current.1 + 1, current.2);
        if let Some(bits) = lava.get_mut(&next) {
            *bits |= 1 << 3;
        } else {
            flood_fill(lava, visited, minimum, maximum, next);
        }
    }

    if current.2 > minimum.2 {
        let next = (current.0, current.1, current.2 - 1);
        if let Some(bits) = lava.get_mut(&next) {
            *bits |= 1 << 4;
        } else {
            flood_fill(lava, visited, minimum, maximum, next);
        }
    }

    if current.2 < maximum.2 {
        let next = (current.0, current.1, current.2 + 1);
        if let Some(bits) = lava.get_mut(&next) {
            *bits |= 1 << 5;
        } else {
            flood_fill(lava, visited, minimum, maximum, next);
        }
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut lava = HashMap::<(i32, i32, i32), u8>::new();

    for line in input.lines() {
        let x: i32;
        let y: i32;
        let z: i32;
        scan!(line.bytes() => "{},{},{}", x, y, z);
        lava.insert((x, y, z), 0);
    }

    let mut interior_exposed = 0;

    for (droplet, _) in lava.iter() {
        if !lava.contains_key(&(droplet.0 - 1, droplet.1, droplet.2)) {
            interior_exposed += 1;
        }
        if !lava.contains_key(&(droplet.0 + 1, droplet.1, droplet.2)) {
            interior_exposed += 1;
        }
        if !lava.contains_key(&(droplet.0, droplet.1 - 1, droplet.2)) {
            interior_exposed += 1;
        }
        if !lava.contains_key(&(droplet.0, droplet.1 + 1, droplet.2)) {
            interior_exposed += 1;
        }
        if !lava.contains_key(&(droplet.0, droplet.1, droplet.2 - 1)) {
            interior_exposed += 1;
        }
        if !lava.contains_key(&(droplet.0, droplet.1, droplet.2 + 1)) {
            interior_exposed += 1;
        }
    }

    let mut minimum = (i32::MAX, i32::MAX, i32::MAX);
    let mut maximum = (i32::MIN, i32::MIN, i32::MIN);

    for (droplet, _) in lava.iter() {
        minimum.0 = minimum.0.min(droplet.0);
        minimum.1 = minimum.1.min(droplet.1);
        minimum.2 = minimum.2.min(droplet.2);
        maximum.0 = maximum.0.max(droplet.0);
        maximum.1 = maximum.1.max(droplet.1);
        maximum.2 = maximum.2.max(droplet.2);
    }

    let minimum = (minimum.0 - 1, minimum.2 - 1, minimum.2 - 1);
    let maximum = (maximum.0 + 1, maximum.2 + 1, maximum.2 + 1);
    let mut visited = HashSet::new();

    flood_fill(&mut lava, &mut visited, minimum, maximum, minimum);

    let exterior_exposed = lava.iter().map(|(_, sides)| sides.count_ones() as usize).sum();

    (interior_exposed, exterior_exposed)
}