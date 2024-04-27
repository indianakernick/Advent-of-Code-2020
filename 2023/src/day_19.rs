use std::collections::HashMap;

use crate::common;

pub fn solve(input: &str) -> (u32, u64) {
    let mut line_iter = common::lines_iter(input);
    let mut workflow_map = WorkflowMap::default();

    for line in &mut line_iter {
        if line.is_empty() { break; }
        workflow_map.insert_from_bytes(line);
    }

    let start_workflow = workflow_map.start();
    let mut accept_sum = 0;

    for line in &mut line_iter {
        let part = Part::from_bytes(line);
        if workflow_map.accepts(&part, start_workflow) {
            accept_sum += part.sum() as u32;
        }
    }

    let range = PartRange {
        lower: Part::new_all(0),
        upper: Part::new_all(4001),
    };

    (accept_sum, workflow_map.accepts_count(range, start_workflow, 0))
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

#[derive(Clone, Copy)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn new_all(all: u16) -> Self {
        Part { x: all, m: all, a: all, s: all }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
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

    fn with<F: FnOnce(u16) -> u16>(&self, rating: Rating, transform: F) -> Self {
        let mut copy = *self;
        match rating {
            Rating::X => copy.x = transform(self.x),
            Rating::M => copy.m = transform(self.m),
            Rating::A => copy.a = transform(self.a),
            Rating::S => copy.s = transform(self.s),
        };
        copy
    }

    fn join<F: FnMut(u16, u16)>(&self, other: &Part, mut apply: F) {
        apply(self.x, other.x);
        apply(self.m, other.m);
        apply(self.a, other.a);
        apply(self.s, other.s);
    }
}

struct PartRange {
    lower: Part,
    upper: Part,
}

impl PartRange {
    fn split(self, rating: Rating, predicate: Predicate) -> (Self, Self) {
        match predicate {
            Predicate::GreaterThan(lower) => (
                Self {
                    lower: self.lower.with(rating, |old| old.max(lower)),
                    upper: self.upper,
                },
                Self {
                    lower: self.lower,
                    upper: self.upper.with(rating, |old| old.min(lower + 1)),
                },
            ),
            Predicate::LessThan(upper) => (
                Self {
                    lower: self.lower,
                    upper: self.upper.with(rating, |old| old.min(upper)),
                },
                Self {
                    lower: self.lower.with(rating, |old| old.max(upper - 1)),
                    upper: self.upper,
                }
            ),
        }
    }

    fn count(self) -> u64 {
        let mut product = 1;
        self.lower.join(&self.upper, |lower, upper| {
            product *= (upper - lower - 1) as u64;
        });
        product
    }
}

#[derive(Clone, Copy)]
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

    fn check(&self, value: u16) -> bool {
        match *self {
            Self::GreaterThan(rhs) => value > rhs,
            Self::LessThan(rhs) => value < rhs,
        }
    }
}

#[derive(Clone, Copy)]
enum WorkflowRef<'a> {
    Accept,
    Reject,
    Workflow(&'a [u8]),
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

    fn check(&self, part: &Part) -> bool {
        self.predicate.check(part.get(self.rating))
    }
}

struct Workflow<'a> {
    // Can we avoid using Vec here?
    // Perhaps an array or a slice onto something like a C++ std::deque.
    // Or perhaps we turn this whole thing into a proper binary tree?
    rules: Vec<Rule<'a>>,
    default: WorkflowRef<'a>,
}

impl<'a> Workflow<'a> {
    fn resolve_next(&self, part: &Part) -> WorkflowRef<'a> {
        for rule in self.rules.iter() {
            if rule.check(part) {
                return rule.next;
            }
        }
        self.default
    }
}

#[derive(Default)]
struct WorkflowMap<'a> {
    map: HashMap<&'a [u8], Workflow<'a>>,
}

impl<'a> WorkflowMap<'a> {
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

    fn get(&self, name: &'a [u8]) -> &Workflow<'a> {
        self.map.get(name).unwrap()
    }

    fn start(&self) -> &Workflow<'a> {
        self.get(b"in".as_slice())
    }

    fn accepts(&self, part: &Part, workflow: &Workflow) -> bool {
        match workflow.resolve_next(&part) {
            WorkflowRef::Accept => true,
            WorkflowRef::Reject => false,
            WorkflowRef::Workflow(name) => self.accepts(part, self.get(name)),
        }
    }

    fn accepts_count(&self,
        range: PartRange,
        workflow: &Workflow,
        rule_index: usize,
    ) -> u64 {
        if rule_index == workflow.rules.len() {
            return match workflow.default {
                WorkflowRef::Accept => range.count(),
                WorkflowRef::Reject => 0,
                WorkflowRef::Workflow(name) => self.accepts_count(range, self.get(name), 0),
            };
        }

        let rule = &workflow.rules[rule_index];
        let (true_range, false_range) = range.split(rule.rating, rule.predicate);
        let false_count = self.accepts_count(false_range, workflow, rule_index + 1);

        match rule.next {
            WorkflowRef::Accept => true_range.count() + false_count,
            WorkflowRef::Reject => false_count,
            WorkflowRef::Workflow(name)
                => self.accepts_count(true_range, self.get(name), 0) + false_count,
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
    assert_eq!(output.1, 167409079868000);
}
