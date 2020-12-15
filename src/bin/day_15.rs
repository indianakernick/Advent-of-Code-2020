fn part_one() -> i32 {
    let mut start: Vec::<i32> = vec![-1,9,12,1,4,17,0,18];
    //let mut start: Vec::<i32> = vec![-1, 2, 1, 3];
    for t in (start.len() - 1)..2020 {
        let mut found = false;
        let mut i = t - 1;
        while i > 0 {
            if start[i as usize] == start[t as usize] {
                start.push((t - i) as i32);
                found = true;
                break;
            }
            i -= 1;
        }
        if !found {
            start.push(0);
        }
    }
    start[start.len() - 1]
}

fn part_two() -> i32 {
    //let mut start: Vec::<i32> = vec![-1,9,12,1,4,17,0,18];
    let mut start = std::collections::HashMap::<i32, Vec::<i32>>::new();
    start.insert(9, vec![1]);
    start.insert(12, vec![2]);
    start.insert(1, vec![3]);
    start.insert(4, vec![4]);
    start.insert(17, vec![5]);
    start.insert(0, vec![6]);
    start.insert(18, vec![7]);
    let mut last = 18;

    for t in 8..30000001 {
        if start.contains_key(&last) {
            if start[&last].len() == 1 {
                last = 0;
                start.entry(last).or_default().push(t);
            } else {
                last = start[&last][start[&last].len() - 1] - start[&last][start[&last].len() - 2];
                start.entry(last).or_default().push(t);
            }
        } else {
            start.insert(last, vec![t]);
        }
    }
    for (key, val) in start {
        if val.contains(&30000000) {
            return key;
        }
    }
    panic!();
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
