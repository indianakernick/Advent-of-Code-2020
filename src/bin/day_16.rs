use text_io::scan;
use adventofcode2020::*;

type Range = std::ops::RangeInclusive::<i32>;

struct Field {
    name: String,
    first: Range,
    second: Range,
    candidate_orders: Vec::<usize>,
    final_order: usize
}

impl Field {
    fn new(name: String, first: Range, second: Range) -> Field {
        Field {
            name,
            first,
            second,
            candidate_orders: Vec::new(),
            final_order: usize::MAX,
        }
    }

    fn contains(&self, value: i32) -> bool {
        self.first.contains(&value) || self.second.contains(&value)
    }

    fn append_order(&mut self, order: usize) {
        self.candidate_orders.push(order);
    }

    fn remove_order(&mut self, order: usize) {
        if let Ok(index) = self.candidate_orders.binary_search(&order) {
            self.candidate_orders.remove(index);
        }
    }
}

fn parse_field(line: String) -> Field {
    let name: String;
    let min_a: i32;
    let max_a: i32;
    let min_b: i32;
    let max_b: i32;
    scan!(line.bytes() => "{}: {}-{} or {}-{}", name, min_a, max_a, min_b, max_b);
    Field::new(name, min_a..=max_a, min_b..=max_b)
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
        for position in 0..tickets[0].len() {
            if tickets.iter().all(|ticket| field.contains(ticket[position])) {
                field.append_order(position);
            }
        }
    }

    for _ in 0..fields.len() {
        let mut found_order = 0;
        for field in fields.iter_mut() {
            if field.candidate_orders.len() == 1 {
                field.final_order = field.candidate_orders[0];
                found_order = field.final_order;
                break;
            }
        }
        fields.iter_mut().for_each(|field| field.remove_order(found_order));
    }

    let mut product = 1u64;
    for field in fields.iter() {
        if field.name.starts_with("departure") {
            product *= tickets[0][field.final_order] as u64;
        }
    }
    println!("Part two: {}", product);
}
