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

#[derive(Parser)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(u8).range(1..=12))]
    day: u8
}

fn print_part(num: u8, output: String) {
    print!("Part {}:", num);
    let sep = if output.contains("\n") { '\n' } else { ' ' };
    println!("{}{}", sep, output);
}

fn print_output<P1: Display, P2: Display>(output: (P1, P2)) {
    print_part(1, output.0.to_string());
    print_part(2, output.1.to_string());
}

fn main() {
    let cli = Cli::parse();
    let input = std::fs::read_to_string(format!("input/day_{:02}.txt", cli.day)).unwrap();

    match cli.day {
         1 => print_output(day_01::solve(&input)),
         2 => print_output(day_02::solve(&input)),
         3 => print_output(day_03::solve(&input)),
         4 => print_output(day_04::solve(&input)),
         5 => print_output(day_05::solve(&input)),
         6 => print_output(day_06::solve(&input)),
         7 => print_output(day_07::solve(&input)),
         8 => print_output(day_08::solve(&input)),
         9 => print_output(day_09::solve(&input)),
        10 => print_output(day_10::solve(&input)),
        11 => print_output(day_11::solve(&input)),
        12 => print_output(day_12::solve(&input)),
        _ => unreachable!(),
    }
}
