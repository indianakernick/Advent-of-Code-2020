use std::collections::{HashSet, HashMap};

pub fn solve(input: &str) -> (i32, usize) {
    let mut elves = HashSet::<(i32, i32)>::new();
    let mut elves2 = HashSet::<(i32, i32)>::new();
    let mut proposals = HashMap::<(i32, i32), Vec::<(i32, i32)>>::new();
    let mut direction_offset = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    for _ in 0..10 {
        for elf in elves.iter() {
            let any = elves.contains(&(elf.0 + -1, elf.1 + -1))
                || elves.contains(&(elf.0 + 0, elf.1 + -1))
                || elves.contains(&(elf.0 + 1, elf.1 + -1))
                || elves.contains(&(elf.0 + 1, elf.1 + 0))
                || elves.contains(&(elf.0 + 1, elf.1 + 1))
                || elves.contains(&(elf.0 + 0, elf.1 + 1))
                || elves.contains(&(elf.0 + -1, elf.1 + 1))
                || elves.contains(&(elf.0 + -1, elf.1 + 0));
            if !any {
                proposals.insert(*elf, vec![*elf]);
                continue;
            }

            let mut moved = false;

            for d in 0..4 {
                if (d + direction_offset) % 4 == 0 {
                    // North
                    if !elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !elves.contains(&(elf.0, elf.1 - 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                    {
                        proposals.entry((elf.0, elf.1 - 1))
                            .and_modify(|v| v.push(*elf))
                            .or_insert(vec![*elf]);
                        moved = true;
                        break;
                    }
                }

                if (d + direction_offset) % 4 == 1 {
                    // South
                    if !elves.contains(&(elf.0 - 1, elf.1 + 1))
                        && !elves.contains(&(elf.0, elf.1 + 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        proposals.entry((elf.0, elf.1 + 1))
                            .and_modify(|v| v.push(*elf))
                            .or_insert(vec![*elf]);
                        moved = true;
                        break;
                    }
                }

                if (d + direction_offset) % 4 == 2 {
                    // West
                    if !elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !elves.contains(&(elf.0 - 1, elf.1))
                        && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                    {
                        proposals.entry((elf.0 - 1, elf.1))
                            .and_modify(|v| v.push(*elf))
                            .or_insert(vec![*elf]);
                        moved = true;
                        break;
                    }
                }

                if (d + direction_offset) % 4 == 3 {
                    // East
                    if !elves.contains(&(elf.0 + 1, elf.1 - 1))
                        && !elves.contains(&(elf.0 + 1, elf.1))
                        && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        proposals.entry((elf.0 + 1, elf.1))
                            .and_modify(|v| v.push(*elf))
                            .or_insert(vec![*elf]);
                        moved = true;
                        break;
                    }
                }
            }

            if !moved {
                proposals.insert(*elf, vec![*elf]);
            }
        }

        for (destination, sources) in proposals.iter() {
            if sources.len() > 1 {
                for elf in sources.iter() {
                    elves2.insert(*elf);
                }
            } else {
                elves2.insert(*destination);
            }
        }

        std::mem::swap(&mut elves, &mut elves2);
        elves2.clear();
        proposals.clear();
        direction_offset += 1;
    }

    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);

    for elf in elves.iter() {
        min = (min.0.min(elf.0), min.1.min(elf.1));
        max = (max.0.max(elf.0), max.1.max(elf.1));
    }

    let area = (max.0 - min.0 + 1) * (max.1 - min.1 + 1) - elves.len() as i32;

    (area, 0)
}
