use std::collections::HashSet;

use text_io::scan;

pub fn solve(input: &str) -> (usize, usize) {
    let mut lava = HashSet::<(i32, i32, i32)>::new();

    for line in input.lines() {
        let x: i32;
        let y: i32;
        let z: i32;
        scan!(line.bytes() => "{},{},{}", x, y, z);
        lava.insert((x, y, z));
    }

    let mut exposed = 0;

    for droplet in lava.iter() {
        if !lava.contains(&(droplet.0 - 1, droplet.1, droplet.2)) {
            exposed += 1;
        }
        if !lava.contains(&(droplet.0 + 1, droplet.1, droplet.2)) {
            exposed += 1;
        }
        if !lava.contains(&(droplet.0, droplet.1 - 1, droplet.2)) {
            exposed += 1;
        }
        if !lava.contains(&(droplet.0, droplet.1 + 1, droplet.2)) {
            exposed += 1;
        }
        if !lava.contains(&(droplet.0, droplet.1, droplet.2 - 1)) {
            exposed += 1;
        }
        if !lava.contains(&(droplet.0, droplet.1, droplet.2 + 1)) {
            exposed += 1;
        }
    }

    (exposed, 0)
}