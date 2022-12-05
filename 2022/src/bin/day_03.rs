use advent_of_code_2022 as util;

fn priority_for_item(item: u8) -> u64 {
    if item.is_ascii_lowercase() {
        (item - b'a' + 1) as u64
    } else {
        (item - b'A' + 27) as u64
    }
}

fn main() {
    const GROUP_SIZE: usize = 3;

    let mut misplaced_sum = 0u64;
    let mut group_sum = 0u64;
    let mut group_members: [u64; GROUP_SIZE] = Default::default();
    let mut member_index = 0usize;

    util::each_line("input/day_03.txt", |line| {
        let first = line[..line.len() / 2].as_bytes();
        let second = line[line.len() / 2..].as_bytes();
        let mut first_set = 0u64;
        let mut second_set = 0u64;

        for item in first.iter() {
            first_set |= 1 << priority_for_item(*item);
        }
        for item in second.iter() {
            second_set |= 1 << priority_for_item(*item);
        }

        let compartment_common_set = first_set & second_set;

        if compartment_common_set != 0 {
            misplaced_sum += compartment_common_set.trailing_zeros() as u64;
        }

        group_members[member_index] = first_set | second_set;

        if member_index == GROUP_SIZE - 1 {
            let mut group_common_set = group_members[0];

            for i in 1..GROUP_SIZE {
                group_common_set &= group_members[i];
            }

            if group_common_set != 0 {
                group_sum += group_common_set.trailing_zeros() as u64
            }
        }

        member_index += 1;

        if member_index == GROUP_SIZE {
            member_index = 0;
        }
    });

    println!("Part 1: {}", misplaced_sum);
    println!("Part 2: {}", group_sum);
}
