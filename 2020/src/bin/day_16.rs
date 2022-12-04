use text_io::scan;
use adventofcode2020::*;

struct Field {
    name: String,
    a_min: i32,
    a_max: i32,
    b_min: i32,
    b_max: i32,
    candidate_positions: Vec::<usize>,
    position: usize,
}

impl Field {
    fn contains(&self, value: i32) -> bool {
        (self.a_min <= value && value <= self.a_max) ||
        (self.b_min <= value && value <= self.b_max)
    }

    fn append_pos(&mut self, pos: usize) {
        self.candidate_positions.push(pos);
    }

    fn remove_pos(&mut self, pos: usize) {
        if let Ok(index) = self.candidate_positions.binary_search(&pos) {
            self.candidate_positions.remove(index);
        }
    }
}

fn parse_field(line: String) -> Field {
    let name: String;
    let a_min: i32;
    let a_max: i32;
    let b_min: i32;
    let b_max: i32;
    scan!(line.bytes() => "{}: {}-{} or {}-{}", name, a_min, a_max, b_min, b_max);
    Field {
        name, a_min, a_max, b_min, b_max,
        candidate_positions: Vec::new(),
        position: usize::MAX,
    }
}

fn parse_ticket(line: String) -> Vec<i32> {
    line.split(',').map(|value| value.parse().unwrap()).collect()
}

fn parse_fields(line_iter: &mut impl Iterator<Item = String>) -> Vec<Field> {
    line_iter.take_while(|line| !line.is_empty()).map(parse_field).collect()
}

fn parse_tickets(line_iter: &mut impl Iterator<Item = String>) -> Vec<Vec<i32>> {
    line_iter.next(); // your ticket:

    let mut tickets = vec![parse_ticket(line_iter.next().unwrap())];

    line_iter.next(); // (blank line)
    line_iter.next(); // nearby tickets:

    for line in line_iter {
        tickets.push(parse_ticket(line));
    }

    tickets
}

fn main() {
    let mut line_iter = line_iter_from_file("input/day_16.txt");
    let mut fields = parse_fields(&mut line_iter);
    let tickets = parse_tickets(&mut line_iter);

    let mut error_rate = 0;

    let tickets = tickets.iter().filter(|ticket| {
        let mut valid = true;
        for ticket_field in ticket.iter() {
            if !fields.iter().any(|field| field.contains(*ticket_field)) {
                error_rate += ticket_field;
                valid = false;
            }
        }
        valid
    }).collect::<Vec<_>>();

    println!("Part one: {}", error_rate);

    for field in fields.iter_mut() {
        for pos in 0..tickets[0].len() {
            if tickets.iter().all(|ticket| field.contains(ticket[pos])) {
                field.append_pos(pos);
            }
        }
    }

    for _ in 0..fields.len() {
        for f in 0..fields.len() {
            if fields[f].candidate_positions.len() == 1 {
                let position = fields[f].candidate_positions.remove(0);
                fields[f].position = position;
                fields.iter_mut().for_each(|field| field.remove_pos(position));
                break;
            }
        }
    }

    let mut product = 1u64;
    for field in fields.iter() {
        if field.name.starts_with("departure") {
            product *= tickets[0][field.position] as u64;
        }
    }

    println!("Part two: {}", product);
}
