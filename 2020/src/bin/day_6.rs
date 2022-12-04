use adventofcode2020::*;

fn main() {
    let mut union_count: u32 = 0;
    let mut inter_count: u32 = 0;
    let mut union_set: u32 = 0;
    let mut inter_set: u32 = (1 << 26) - 1;

    lines_from_file("input/day_6.txt", |line| {
        if line.is_empty() {
            union_count += union_set.count_ones();
            inter_count += inter_set.count_ones();
            union_set = 0;
            inter_set = (1 << 26) - 1;
        } else {
            let mut line_set: u32 = 0;
            for ch in line.bytes() {
                line_set |= 1 << (ch - b'a');
            }
            union_set |= line_set;
            inter_set &= line_set;
        }
    });

    println!("Part one: {}", union_count);
    println!("Part two: {}", inter_count);
}
