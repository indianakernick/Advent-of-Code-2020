use std::collections::HashMap;

use crate::common;

pub fn solve(input: &str) -> (u32, u32) {
    let mut line_iter = common::lines_iter(input);
    let mut workflows = Workflows::default();

    for line in &mut line_iter {
        if line.is_empty() { break; }
        workflows.insert_from_bytes(line);
    }

    let start_workflow = workflows.start();
    let mut accept_sum = 0;

    for line in &mut line_iter {
        let part = Part::from_bytes(line);
        if workflows.evaluate(&part, start_workflow) {
            accept_sum += part.sum() as u32;
        }
    }

    (accept_sum, 0)
}

#[derive(Clone, Copy)]
enum Rating {
    X,
    M,
    A,
    S,
}

impl Rating {
    fn from_byte(b: u8) -> Rating {
        match b {
            b'x' => Self::X,
            b'm' => Self::M,
            b'a' => Self::A,
            b's' => Self::S,
            _ => panic!("Invalid input"),
        }
    }
}

struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn from_bytes(bytes: &[u8]) -> Part {
        let comma_1 = common::index_of(bytes, b',');
        let comma_2 = common::index_of_after(bytes, b',', comma_1 + 1);
        let comma_3 = common::index_of_after(bytes, b',', comma_2 + 1);

        Self {
            x: common::parse_u32(&bytes[3..comma_1]) as u16,
            m: common::parse_u32(&bytes[comma_1 + 3..comma_2]) as u16,
            a: common::parse_u32(&bytes[comma_2 + 3..comma_3]) as u16,
            s: common::parse_u32(&bytes[comma_3 + 3..bytes.len() - 1]) as u16,
        }
    }

    fn get(&self, rating: Rating) -> u16 {
        match rating {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }

    fn sum(&self) -> u16 {
        self.x + self.m + self.a + self.s
    }
}

enum Predicate {
    GreaterThan(u16),
    LessThan(u16),
}

impl Predicate {
    fn from_bytes(bytes: &[u8]) -> (Self, usize) {
        let colon = common::index_of_after(bytes, b':', 1);
        let value = common::parse_u32(&bytes[1..colon]) as u16;
        let length = colon + 1;
        match bytes[0] {
            b'>' => (Self::GreaterThan(value), length),
            b'<' => (Self::LessThan(value), length),
            _ => panic!("Invalid input"),
        }
    }

    fn evaluate(&self, value: u16) -> bool {
        match *self {
            Self::GreaterThan(rhs) => value > rhs,
            Self::LessThan(rhs) => value < rhs,
        }
    }
}

#[derive(Clone, Copy)]
enum WorkflowRef<'a> {
    Workflow(&'a [u8]),
    Accept,
    Reject,
}

impl<'a> WorkflowRef<'a> {
    fn from_bytes(bytes: &'a [u8]) -> (Self, usize) {
        let comma = bytes.iter().position(|b| *b == b',').unwrap_or(bytes.len());
        let length = comma + 1;
        match &bytes[..comma] {
            b"A" => (Self::Accept, length),
            b"R" => (Self::Reject, length),
            name @ _ => (Self::Workflow(name), length),
        }
    }
}

struct Rule<'a> {
    rating: Rating,
    predicate: Predicate,
    next: WorkflowRef<'a>,
}

impl<'a> Rule<'a> {
    fn from_bytes(bytes: &'a [u8]) -> (Self, usize) {
        let rating = Rating::from_byte(bytes[0]);
        let (predicate, p_length) = Predicate::from_bytes(&bytes[1..]);
        let (next, r_length) = WorkflowRef::from_bytes(&bytes[1 + p_length..]);
        (Self { rating, predicate, next }, 1 + p_length + r_length)
    }

    fn evaluate(&self, part: &Part) -> bool {
        self.predicate.evaluate(part.get(self.rating))
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default: WorkflowRef<'a>,
}

impl<'a> Workflow<'a> {
    fn evaluate(&self, part: &Part) -> WorkflowRef<'a> {
        for rule in self.rules.iter() {
            if rule.evaluate(part) {
                return rule.next;
            }
        }
        self.default
    }
}

#[derive(Default)]
struct Workflows<'a> {
    map: HashMap<&'a [u8], Workflow<'a>>,
}

impl<'a> Workflows<'a> {
    fn insert_from_bytes(&mut self, bytes: &'a [u8]) {
        let open_curly = common::index_of(bytes, b'{');
        let last_comma = common::rindex_of(bytes, b',');

        let mut rules = Vec::new();
        let mut rule_index = open_curly + 1;

        loop {
            let (rule, length) = Rule::from_bytes(&bytes[rule_index..]);
            rules.push(rule);
            rule_index += length;
            if rule_index == last_comma + 1 { break; }
        }

        let (default, _) = WorkflowRef::from_bytes(&bytes[rule_index..bytes.len() - 1]);

        self.map.insert(&bytes[..open_curly], Workflow { rules, default });
    }

    fn start<'b>(&'b self) -> &'b Workflow<'a> {
        self.map.get(b"in".as_slice()).unwrap()
    }

    fn evaluate(&self, part: &Part, workflow: &Workflow) -> bool {
        match workflow.evaluate(&part) {
            WorkflowRef::Accept => true,
            WorkflowRef::Reject => false,
            WorkflowRef::Workflow(name) => self.evaluate(part, self.map.get(name).unwrap()),
        }
    }
}

#[cfg(test)]
#[test]
fn example() {
    let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let output = solve(input);
    assert_eq!(output.0, 19114);
}
