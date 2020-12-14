use adventofcode2020::*;

fn part_one() {
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    let mut facing: i32 = 1;

    lines_from_file("input/day_12.txt", |line| {
        let string = line.as_str();
        let value = string[1..].parse::<i32>().unwrap();
        match &string[..1] {
            "N" => pos_y += value,
            "S" => pos_y -= value,
            "E" => pos_x += value,
            "W" => pos_x -= value,
            "L" => facing = (4 + facing - value / 90) % 4,
            "R" => facing = (4 + facing + value / 90) % 4,
            "F" => match facing {
                0 => pos_y += value,
                1 => pos_x += value,
                2 => pos_y -= value,
                3 => pos_x -= value,
                _ => panic!()
            },
            _ => panic!()
        }
    });

    println!("Part one: {}", pos_x.abs() + pos_y.abs());
}

fn part_two() {
    let mut way_x: i32 = 10;
    let mut way_y: i32 = 1;
    let mut ship_x: i32 = 0;
    let mut ship_y: i32 = 0;

    let rotate = |way_x: &mut i32, way_y: &mut i32, turns| {
        for _ in 0..turns {
            *way_x *= -1;
            std::mem::swap(way_x, way_y);
        }
    };

    lines_from_file("input/day_12.txt", |line| {
        let string = line.as_str();
        let value = string[1..].parse::<i32>().unwrap();
        match &string[..1] {
            "N" => way_y += value,
            "S" => way_y -= value,
            "E" => way_x += value,
            "W" => way_x -= value,
            "L" => rotate(&mut way_x, &mut way_y, 4 - value / 90),
            "R" => rotate(&mut way_x, &mut way_y, value / 90),
            "F" => {
                ship_x += value * way_x;
                ship_y += value * way_y;
            },
            _ => panic!()
        }
    });

    println!("Part two: {}", ship_x.abs() + ship_y.abs());
}

fn main() {
    part_one();
    part_two();
}
