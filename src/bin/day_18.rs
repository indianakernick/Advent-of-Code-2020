use adventofcode2020::*;

#[derive(PartialEq)]
enum Token {
    Digit(u8),
    Add,
    Mul,
    LParen,
}

fn parse(line: &String, add_high_prec: bool) -> Vec<Token> {
    let mut output = Vec::<Token>::new();
    let mut operators = Vec::<Token>::new();

    for ch in line.chars() {
        match ch {
            ' ' => continue,
            '0'..='9' => output.push(Token::Digit(ch as u8 - '0' as u8)),
            '+' => {
                if !add_high_prec {
                    while !operators.is_empty() && *operators.last().unwrap() != Token::LParen {
                        output.push(operators.pop().unwrap());
                    }
                }
                operators.push(Token::Add)
            },
            '*' => {
                while !operators.is_empty() && *operators.last().unwrap() != Token::LParen {
                    output.push(operators.pop().unwrap());
                }
                operators.push(Token::Mul);
            },
            '(' => operators.push(Token::LParen),
            ')' => {
                while *operators.last().unwrap() != Token::LParen {
                    output.push(operators.pop().unwrap());
                }
                if *operators.last().unwrap() == Token::LParen {
                    operators.pop();
                }
            },
            _ => panic!()
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

fn evaluate(tokens: Vec<Token>) -> u64 {
    let mut eval_stack = Vec::<u64>::new();

    for token in tokens.iter() {
        match token {
            Token::Digit(d) => eval_stack.push(*d as u64),
            Token::Add => {
                let a = eval_stack.pop().unwrap();
                let b = eval_stack.pop().unwrap();
                eval_stack.push(a + b);
            },
            Token::Mul => {
                let a = eval_stack.pop().unwrap();
                let b = eval_stack.pop().unwrap();
                eval_stack.push(a * b);
            },
            Token::LParen => panic!()
        }
    }

    *eval_stack.first().unwrap()
}

fn main() {
    let mut sum_one = 0;
    let mut sum_two = 0;
    lines_from_file("input/day_18.txt", |line| {
        sum_one += evaluate(parse(line, false));
        sum_two += evaluate(parse(line, true));
    });
    println!("Part one: {}", sum_one);
    println!("Part two: {}", sum_two);
}
