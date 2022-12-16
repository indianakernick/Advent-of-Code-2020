use text_io::scan;

fn get_short_long_map(valves: &[(u32, Vec<usize>)], start: usize) -> (Vec<usize>, Vec<usize>) {
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

const UNREACHABLE: u32 = 64; // u16::MAX as u32;

fn get_distances(valves: &[(u32, Vec<usize>)]) -> Vec<Vec<u32>> {
    let mut distance = Vec::<Vec<u32>>::new();
    distance.resize_with(valves.len(), || vec![UNREACHABLE; valves.len()]);

    for (i, valve) in valves.iter().enumerate() {
        for neighbor in valve.1.iter() {
            distance[i][*neighbor] = 1;
            distance[*neighbor][i] = 1;
        }
    }

    // Floyd-Warshall
    for i in 0..distance.len() {
        for j in 0..distance.len() {
            for k in 0..distance.len() {
                distance[j][k] = distance[j][k].min(distance[j][i] + distance[i][k]);
            }
        }
    }

    distance
}


fn search_inner(
    table: &mut Vec<Vec<Vec<u32>>>,
    valves: &[(u32, Vec<usize>)],
    short_to_long: &[usize],
    long_to_short: &[usize],
    distances: &[Vec<u32>],
    current_long: usize,
    remaining: u32,
    enabled: u64,
) -> u32 {
    if remaining == 0 {
        return 0;
    }

    let current_short = long_to_short[current_long];
    let memoized = table[remaining as usize][current_short][enabled as usize];

    if memoized != u32::MAX {
        return memoized;
    }

    // Maximum score from visiting neighbors if we decide not to enable the
    // current valve.
    let mut skip_neighbor_score = 0;

    for neighbor_long in short_to_long.iter() {
        // Checking if neighbor has a positive flow rate
        if valves[*neighbor_long].0 == 0 { continue }
        // Checking if we'll reach that neighbor with enough time to turn on the
        // valve and then let it flow.
        let neighbor_distance = distances[current_long][*neighbor_long];
        if neighbor_distance + 1 >= remaining { continue }

        skip_neighbor_score = skip_neighbor_score.max(search_inner(
            table,
            valves,
            short_to_long,
            long_to_short,
            distances,
            *neighbor_long,
            remaining - neighbor_distance,
            enabled,
        ));
    }

    if current_short == 0 || current_short == usize::MAX {
        table[remaining as usize][current_short][enabled as usize] = skip_neighbor_score;
        return skip_neighbor_score;
    }

    let bit = 1 << current_short;

    let result = if remaining >= 2 && (enabled & bit) == 0 {
        let remaining = remaining - 1;
        let enabled = enabled | bit;

        let current_score = valves[current_long].0 * remaining;
        let next_score = search_inner(
            table,
            valves,
            short_to_long,
            long_to_short,
            distances,
            current_long,
            remaining,
            enabled,
        );

        skip_neighbor_score.max(current_score + next_score)
    } else {
        skip_neighbor_score
    };

    table[remaining as usize][current_short][enabled as usize] = result;
    result
}

fn search(valves: &[(u32, Vec<usize>)], start: usize) -> u32 {
    let (short_to_long, long_to_short) = get_short_long_map(valves, start);
    let distances = get_distances(valves);

    // [remaining][position][enabled]
    let mut table = Vec::<Vec<Vec<u32>>>::new();

    table.resize_with(31, || {
        let mut v = Vec::new();
        v.resize_with(valves.len(), || vec![u32::MAX; 1 << short_to_long.len()]);
        v
    });

    search_inner(&mut table, valves, &short_to_long, &long_to_short, &distances, start, 30, 0)
}

fn to_id(s: [u8; 2]) -> u16 {
    ((s[0] as u16) << 8) | s[1] as u16
}

pub fn solve(input: &str) -> (u32, usize) {
    let mut valves = Vec::<(u16, u32, Vec<u16>)>::new();

    for line in input.lines() {
        let mut line_bytes = line.bytes();
        let current: String;
        let flow_rate: u32;
        scan!(line_bytes => "Valve {} has flow rate={}; tunnel", current, flow_rate);

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

    (search(&new_valves, start), 0)
}
