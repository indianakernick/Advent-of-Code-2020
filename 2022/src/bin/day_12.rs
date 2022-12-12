use advent_of_code_2022 as util;

fn can_step(from: u8, to: u8) -> bool {
    from >= to || from + 1 == to
}

fn search(
    pos: (usize, usize),
    end_pos: (usize, usize),
    map: &[Vec<u8>]
) -> Option<usize> {
    pathfinding::directed::dijkstra::dijkstra(
        &pos,
        |pos| {
            let curr = map[pos.1][pos.0];
            let mut branches = [None; 4];

            if pos.1 > 0 && can_step(curr, map[pos.1 - 1][pos.0]) {
                branches[0] = Some((pos.0, pos.1 - 1));
            }

            if pos.0 < map[0].len() - 1 && can_step(curr, map[pos.1][pos.0 + 1]) {
                branches[1] = Some((pos.0 + 1, pos.1));
            }

            if pos.1 < map.len() - 1 && can_step(curr, map[pos.1 + 1][pos.0]) {
                branches[2] = Some((pos.0, pos.1 + 1));
            }

            if pos.0 > 0 && can_step(curr, map[pos.1][pos.0 - 1]) {
                branches[3] = Some((pos.0 - 1, pos.1));
            }

            branches.into_iter().filter_map(|b| b.map(|p| (p, 1)))
        },
        |p| p.0 == end_pos.0 && p.1 == end_pos.1
    ).map(|(_, c)| c)
}

fn main() {
    let mut map = Vec::<Vec<u8>>::new();
    let mut pos = (0, 0);
    let mut end_pos = (0, 0);

    util::each_line("input/day_12.txt", |line| {
        if let Some(start_x) = line.bytes().position(|c| c == b'S') {
            pos.0 = start_x;
            pos.1 = map.len();
        }

        if let Some(end_x) = line.bytes().position(|c| c == b'E') {
            end_pos.0 = end_x;
            end_pos.1 = map.len();
        }

        map.push(line.into());
    });

    map[pos.1][pos.0] = b'a';
    map[end_pos.1][end_pos.0] = b'z';

    println!("Part 1: {}", search(pos, end_pos, &map).unwrap());
}
