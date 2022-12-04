use adventofcode2020::*;
use bitflags::bitflags;

bitflags!(
    struct Fields: u8 {
        const BYR = 0b00000001u8;
        const IYR = 0b00000010u8;
        const EYR = 0b00000100u8;
        const HGT = 0b00001000u8;
        const HCL = 0b00010000u8;
        const ECL = 0b00100000u8;
        const PID = 0b01000000u8;
    }
);

fn validate_byr(value: &str) -> bool {
    match value.parse::<i32>() {
        Ok(v) => 1920 <= v && v <= 2002,
        Err(_) => false
    }
}

fn validate_iyr(value: &str) -> bool {
    match value.parse::<i32>() {
        Ok(v) => 2010 <= v && v <= 2020,
        Err(_) => false
    }
}

fn validate_eyr(value: &str) -> bool {
    match value.parse::<i32>() {
        Ok(v) => 2020 <= v && v <= 2030,
        Err(_) => false
    }
}

fn validate_hgt(value: &str) -> bool {
    let last_two = value.len() - 2;
    let unit = &value[last_two..];
    let value = &value[..last_two];
    match value.parse::<i32>() {
        Ok(v) => match unit {
            "cm" => 150 <= v && v <= 193,
            "in" => 59 <= v && v <= 76,
            _ => false
        },
        Err(_) => false
    }
}

fn validate_hcl(value: &str) -> bool {
    value.len() == 7
        && value.chars().nth(0).unwrap() == '#'
        && value.chars().skip(1).all(|ch| ch.is_ascii_hexdigit())
}

fn validate_ecl(value: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
}

fn validate_pid(value: &str) -> bool {
    value.len() == 9 && value.chars().all(|ch| ch.is_ascii_digit())
}

fn validate(key: &str, value: &str) -> (Fields, bool) {
    match key {
        "byr" => (Fields::BYR, validate_byr(value)),
        "iyr" => (Fields::IYR, validate_iyr(value)),
        "eyr" => (Fields::EYR, validate_eyr(value)),
        "hgt" => (Fields::HGT, validate_hgt(value)),
        "hcl" => (Fields::HCL, validate_hcl(value)),
        "ecl" => (Fields::ECL, validate_ecl(value)),
        "pid" => (Fields::PID, validate_pid(value)),
        "cid" => (Fields::empty(), true),
        _ => (Fields::empty(), false)
    }
}

fn main() {
    let mut valid_key_count = 0;
    let mut valid_val_count = 0;
    let mut valid_val = true;
    let mut current = Fields::empty();

    lines_from_file("input/day_4.txt", |line| {
        if line.is_empty() {
            if current == Fields::all() {
                valid_key_count += 1;
                if valid_val {
                    valid_val_count += 1;
                }
            }
            valid_val = true;
            current = Fields::empty();
        } else {
            for pair in line.split(' ') {
                let key = &pair[0..3];
                let value = &pair[4..];
                let (field, valid) = validate(key, value);
                current |= field;
                valid_val = valid_val && valid;
            }
        }
    });

    println!("Part one: {}", valid_key_count);
    println!("Part two: {}", valid_val_count);
}
