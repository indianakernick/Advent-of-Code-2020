fn can_step(from: u8, to: u8) -> bool {
    from >= to || from + 1 == to
}

fn search(
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    map: &[Vec<u8>]
) -> Option<usize> {
    pathfinding::directed::dijkstra::dijkstra(
        &start_pos,
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
        |pos| pos.0 == end_pos.0 && pos.1 == end_pos.1
    ).map(|(_, c)| c)
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut map = Vec::<Vec<u8>>::new();
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let mut low_points = Vec::new();

    for line in input.lines() {
        for (i, c) in line.bytes().enumerate() {
            match c {
                b'S' => start_pos = (i, map.len()),
                b'E' => end_pos = (i, map.len()),
                b'a' => low_points.push((i, map.len())),
                _ => {}
            }
        }

        map.push(line.into());
    }

    map[start_pos.1][start_pos.0] = b'a';
    map[end_pos.1][end_pos.0] = b'z';

    (
        search(start_pos, end_pos, &map).unwrap(),
        low_points
            .iter()
            .filter_map(|pos| search(*pos, end_pos, &map))
            .min()
            .unwrap(),
    )
}
