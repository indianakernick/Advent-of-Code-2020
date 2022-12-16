use text_io::scan;

// We're keeping a short list of valves. The important valves are the ones with
// a positive flow rate and of course the starting valve.
fn get_short_long_map(
    valves: &[(u32, Vec<usize>)],
    start: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut short_to_long = Vec::new();
    let mut long_to_short = vec![usize::MAX; valves.len()];

    short_to_long.push(start);
    long_to_short[start] = 0;

    for (i, valve) in valves.iter().enumerate() {
        if valve.0 > 0 {
            long_to_short[i] = short_to_long.len();
            short_to_long.push(i);
        }
    }

    (short_to_long, long_to_short)
}

const UNREACHABLE: u32 = u16::MAX as u32;

// Precomputing the distances between every pair of short valve.
fn get_distances(valves: &[(u32, Vec<usize>)]) -> Vec<Vec<u32>> {
    let mut dist = Vec::<Vec<u32>>::new();
    dist.resize_with(valves.len(), || vec![UNREACHABLE; valves.len()]);

    for (i, valve) in valves.iter().enumerate() {
        for neighbor in valve.1.iter() {
            dist[i][*neighbor] = 1;
            dist[*neighbor][i] = 1;
        }
    }

    // Floyd-Warshall with some tricks to make it perform well in Rust.
    // https://stackoverflow.com/a/70059224/4093378
    let n = dist.len();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (dist_j, dist_i) = if j < i {
                let (lo, hi) = dist.split_at_mut(i);
                (&mut lo[j][..n], &mut hi[0][..n])
            } else {
                let (lo, hi) = dist.split_at_mut(j);
                (&mut hi[0][..n], &mut lo[i][..n])
            };
            let dist_ji = dist_j[i];
            for k in 0..n {
                dist_j[k] = dist_j[k].min(dist_ji + dist_i[k]);
            }
        }
    }

    dist
}

fn search_inner(
    memoize_table: &mut Vec<Vec<Vec<u32>>>,
    valves: &[(u32, Vec<usize>)],
    short_to_long: &[usize],
    long_to_short: &[usize],
    distances: &[Vec<u32>],
    current_long: usize,
    remaining: u32,
    available: u64,
) -> u32 {
    if remaining == 0 {
        return 0;
    }

    let current_short = long_to_short[current_long];
    let memoized = memoize_table[remaining as usize][current_short][available as usize];

    if memoized != u32::MAX {
        return memoized;
    }

    // At each point, we can either visit another valve or open the current
    // valve.

    let mut score = 0;

    // Skipping the first one because it's the starting valve and that doesn't
    // have a positive flow rate.
    for neighbor_long in short_to_long.iter().skip(1) {
        if *neighbor_long == current_long { continue }
        // Checking if we'll reach that neighbor with enough time to turn on the
        // valve and then let it flow.
        let neighbor_distance = distances[current_long][*neighbor_long];
        if neighbor_distance + 1 >= remaining { continue }

        score = score.max(search_inner(
            memoize_table,
            valves,
            short_to_long,
            long_to_short,
            distances,
            *neighbor_long,
            remaining - neighbor_distance,
            available,
        ));
    }

    let bit = 1 << current_short;

    // Opening the current valve if we can.
    let result = if remaining >= 2 && (available & bit) != 0 {
        let remaining = remaining - 1;
        let available = available & !bit;

        let current_score = valves[current_long].0 * remaining;
        let next_score = search_inner(
            memoize_table,
            valves,
            short_to_long,
            long_to_short,
            distances,
            current_long,
            remaining,
            available,
        );

        score.max(current_score + next_score)
    } else {
        score
    };

    memoize_table[remaining as usize][current_short][available as usize] = result;
    result
}

fn search(valves: &[(u32, Vec<usize>)], start: usize) -> (u32, u32) {
    let (short_to_long, long_to_short) = get_short_long_map(valves, start);
    let distances = get_distances(valves);

    // [remaining][position][available]
    let mut memoize_table = Vec::<Vec<Vec<u32>>>::new();

    memoize_table.resize_with(31, || {
        let mut v = Vec::new();
        v.resize_with(valves.len(), || vec![u32::MAX; 1 << short_to_long.len()]);
        v
    });

    let part_1 = search_inner(
        &mut memoize_table,
        valves,
        &short_to_long,
        &long_to_short,
        &distances,
        start,
        30,
        (1 << short_to_long.len()) - 2
    );

    let short_len = short_to_long.len();
    let mut part_2 = 0;

    // Iterating over two disjoint sets of valves to open.
    for enabled in (0..1 << short_len).step_by(2) {
        for enabled_count in 2..short_len + 1 {
            let count_mask = ((1 << enabled_count) - 1) & !1;
            let me_enabled = enabled & count_mask;
            let el_enabled = !enabled & count_mask;
            let me_score = search_inner(
                &mut memoize_table,
                valves,
                &short_to_long,
                &long_to_short,
                &distances,
                start,
                26,
                me_enabled
            );
            let el_score = search_inner(
                &mut memoize_table,
                valves,
                &short_to_long,
                &long_to_short,
                &distances,
                start,
                26,
                el_enabled
            );

            part_2 = part_2.max(me_score + el_score);
        }
    }

    (part_1, part_2)
}

fn to_id(s: [u8; 2]) -> u16 {
    ((s[0] as u16) << 8) | s[1] as u16
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut valves = Vec::<(u16, u32, Vec<u16>)>::new();

    for line in input.lines() {
        let mut line_bytes = line.bytes();
        let current: String;
        let flow_rate: u32;
        scan!(line_bytes => "Valve {} has flow rate={}; tunnel", current, flow_rate);

        // At this point I realised that maybe regex would be better than this
        // text_io library.
        let mut line_bytes = line_bytes.skip_while(|ch| *ch == b' ' || ch.is_ascii_lowercase());
        let mut next = Vec::<u16>::new();

        while let Some(b1) = line_bytes.next() {
            let b2 = line_bytes.next().unwrap();
            next.push(to_id([b1, b2]));

            line_bytes.next();
            line_bytes.next();
        }

        let current = current.as_bytes();
        valves.push((to_id([current[0], current[1]]), flow_rate, next));
    }

    // Mapping from IDs to indices
    let new_valves = valves.iter()
        .map(|valve| {
            let indices = valve.2.iter()
                .map(|id| {
                    valves.iter().position(|other| *id == other.0).unwrap()
                })
                .collect::<Vec<_>>();
            (valve.1, indices)
        })
        .collect::<Vec<_>>();

    let start = valves.iter().position(|v| v.0 == to_id([b'A', b'A'])).unwrap();

    search(&new_valves, start)
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    let output = solve(input);
    assert_eq!(output.0, 1651);
    assert_eq!(output.1, 1707);
}
