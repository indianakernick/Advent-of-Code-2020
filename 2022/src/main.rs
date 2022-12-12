mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

use clap::Parser;
use std::fmt::Display;

const DAY_COUNT: u8 = 12;

#[derive(Parser)]
struct Cli {
    /// Run a specific day or run all days if unspecified
    #[arg(value_parser = clap::value_parser!(u8).range(1..=DAY_COUNT as i64))]
    day: Option<u8>,
}

fn print_part(num: u8, mut output: String) {
    print!("Part {num}:");
    if output.ends_with('\n') {
        output.pop();
    }
    let sep = if output.contains("\n") { '\n' } else { ' ' };
    println!("{sep}{output}");
}

fn print_output<P1: Display, P2: Display>(output: (P1, P2)) {
    print_part(1, output.0.to_string());
    print_part(2, output.1.to_string());
}

fn solve_and_print(day: u8) {
    match day {
        1 => print_output(day_01::solve(include_str!("../input/day_01.txt"))),
        2 => print_output(day_02::solve(include_str!("../input/day_02.txt"))),
        3 => print_output(day_03::solve(include_str!("../input/day_03.txt"))),
        4 => print_output(day_04::solve(include_str!("../input/day_04.txt"))),
        5 => print_output(day_05::solve(include_str!("../input/day_05.txt"))),
        6 => print_output(day_06::solve(include_str!("../input/day_06.txt"))),
        7 => print_output(day_07::solve(include_str!("../input/day_07.txt"))),
        8 => print_output(day_08::solve(include_str!("../input/day_08.txt"))),
        9 => print_output(day_09::solve(include_str!("../input/day_09.txt"))),
       10 => print_output(day_10::solve(include_str!("../input/day_10.txt"))),
       11 => print_output(day_11::solve(include_str!("../input/day_11.txt"))),
       12 => print_output(day_12::solve(include_str!("../input/day_12.txt"))),
       _ => unreachable!(),
   }
}

fn main() {
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        solve_and_print(day);
    } else {
        for day in 1..=DAY_COUNT {
            println!("Day {day}");
            solve_and_print(day);
            if day != DAY_COUNT {
                println!();
            }
        }
    }
}
