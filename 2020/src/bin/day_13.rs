use adventofcode2020::*;
use std::io::BufRead;

// Didn't actually use this to solve the problem initially
fn parse_input() -> (i32, Vec::<i32>) {
    let mut reader = open_file("input/day_13.txt");
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.pop();
    let depart_time = line.parse::<i32>().unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();
    let bus_ids = line
        .split(',')
        .map(|elem| {
            if elem == "x" {
                -1
            } else {
                elem.parse::<i32>().unwrap()
            }
        })
        .collect::<Vec::<_>>();
    (depart_time, bus_ids)
}

fn part_one(depart_time: i32, bus_ids: &[i32]) -> i32 {
    let mut min_wait_time = i32::MAX;
    let mut min_wait_id = 0;
    for id in bus_ids.iter() {
        if *id == -1 {
            continue;
        }
        let wait_time = *id - depart_time % *id;
        if wait_time < min_wait_time {
            min_wait_time = wait_time;
            min_wait_id = *id;
        }
    }
    min_wait_id * min_wait_time
}

fn main() {
    // let depart_time = 1006697;
    // let bus_ids = [13,-1,-1,41,-1,-1,-1,-1,-1,-1,-1,-1,-1,641,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,19,-1,-1,-1,-1,17,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,29,-1,661,-1,-1,-1,-1,-1,37,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,23];
    let (depart_time, bus_ids) = parse_input();

    println!("Part one: {}", part_one(depart_time, &bus_ids));

    print!("Part two: ");
    for i in 0..bus_ids.len() {
        if bus_ids[i] != -1 {
            print!("(t + {}) mod {} = 0, ", i, bus_ids[i]);
        }
    }
    println!();
}
