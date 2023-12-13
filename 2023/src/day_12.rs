use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut sum = 0;
    let mut arrangement = Vec::new();
    let mut expected_group_sizes = Vec::new();
    let mut actual_group_sizes = Vec::new();

    for line in common::lines_iter(input) {
        let space = line.iter().position(|ch| *ch == b' ').unwrap();
        let arrangement_spec = &line[..space];

        expected_group_sizes.clear();
        expected_group_sizes.extend(line[space + 1..]
            .split(|ch| *ch == b',')
            .map(common::parse_u32));

        let unknown_count = arrangement_spec.iter().filter(|ch| **ch == b'?').count();
        let arrangement_count = 1u32 << unknown_count;

        for arrangement_index in 0..arrangement_count {
            arrangement.clear();

            let mut bit_index = 0;

            arrangement.extend(arrangement_spec.iter().map(|spring| match spring {
                b'#' => true,
                b'.' => false,
                b'?' => {
                    let value = (arrangement_index.overflowing_shr(bit_index).0) & 1 == 1;
                    bit_index += 1;
                    value
                },
                _ => panic!("Invalid input"),
            }));

            actual_group_sizes.clear();

            let mut current = false;
            let mut count = 0;

            for state in arrangement.iter() {
                match (current, state) {
                    (true, true) => count += 1,
                    (true, false) => actual_group_sizes.push(count),
                    (false, true) => count = 1,
                    (false, false) => {}
                }
                current = *state;
            }

            if current {
                actual_group_sizes.push(count);
            }

            if actual_group_sizes == expected_group_sizes {
                sum += 1;
            }
        }
    }

    (sum, 0)
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
}
