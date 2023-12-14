use std::collections::HashMap;

use crate::common;

pub fn solve(input: &str) -> (u64, u64) {
    let mut arrangement_specs = Vec::new();
    let mut group_sizes = Vec::new();

    const COPY: usize = 5;

    for line in common::lines_iter(input) {
        let space = line.iter().position(|ch| *ch == b' ').unwrap();

        let mut arrangement_spec = vec![line[0]];

        // Collapse consecutive operational springs.
        for i in 1..space {
            if line[i - 1] == b'.' && line[i] == b'.' {
                continue;
            }
            arrangement_spec.push(line[i]);
        }

        let spec_len = arrangement_spec.len();

        for _ in 1..COPY {
            arrangement_spec.push(b'?');
            arrangement_spec.extend_from_within(..spec_len);
        }

        arrangement_specs.push(arrangement_spec);

        let mut group_size = line[space + 1..]
            .split(|ch| *ch == b',')
            .map(common::parse_u64)
            .collect::<Vec<_>>();

        let group_size_len = group_size.len();

        for _ in 1..COPY {
            group_size.extend_from_within(..group_size_len);
        }

        group_sizes.push(group_size);
    }

    let length = arrangement_specs.len();
    let mut sum = (0, 0);
    let mut cache = HashMap::new();

    for i in 0..length {
        let spec = &arrangement_specs[i];
        let sizes = &group_sizes[i];
        sum.0 += count_arrangements(
            &mut cache,
            &spec[..spec.len() / COPY],
            &sizes[..sizes.len() / COPY]
        );
        sum.1 += count_arrangements(&mut cache, spec, sizes);
    }

    sum
}

fn count_arrangements<'a, 'b>(
    cache: &'a mut HashMap<(&'b [u8], &'b [u64]), u64>,
    spec: &'b [u8],
    sizes: &'b [u64],
) -> u64 {
    if sizes.len() == 0 {
        return if spec.iter().all(|b| *b == b'.' || *b == b'?') {
            // Assume that the remaining unknowns are operational.
            1
        } else {
            0
        };
    }

    if (spec.len() == 0 && sizes.len() > 0) || spec.len() < sizes[0] as usize {
        return 0;
    }

    if let Some(count) = cache.get(&(spec, sizes)) {
        return *count;
    }

    if !spec[0..sizes[0] as usize].iter().all(|b| *b == b'#' || *b == b'?') {
        let result = if spec[0] == b'.' || spec[0] == b'?' {
            // Assume that the first spring is operational.
            count_arrangements(cache, &spec[1..], sizes)
        } else {
            0
        };
        cache.insert((spec, sizes), result);
        return result;
    }

    if spec.len() > sizes[0] as usize {
        let next = spec[sizes[0] as usize];

        let first = if next == b'.' || next == b'?' {
            // Assume that spring after this contiguous block of broken
            // springs is operational.
            count_arrangements(cache, &spec[sizes[0] as usize + 1..], &sizes[1..])
        } else {
            0
        };

        let second = if spec[0] == b'?' {
            // Assume that the first spring is operational.
            count_arrangements(cache, &spec[1..], sizes)
        } else {
            0
        };

        let result = first + second;
        cache.insert((spec, sizes), result);
        return result;
    }

    let result = count_arrangements(cache, &spec[sizes[0] as usize..], &sizes[1..]);
    cache.insert((spec, sizes), result);
    result
}

#[cfg(test)]
#[test]
fn example() {
    let input =
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let output = solve(input);
    assert_eq!(output.0, 21);
    assert_eq!(output.1, 525152);
}
