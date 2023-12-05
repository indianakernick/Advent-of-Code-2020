macro_rules! declare_days {
    ($($day:ident)+) => {
        $(mod $day;)+

        declare_days!(@solve 0u8, $($day)+,);
    };

    (@solve $index:expr, $first:ident $($rest:ident)*, $(($res_index:expr, $res_ident:ident))*) => {
        declare_days!(@solve $index + 1u8, $($rest)*, ($index + 1u8, $first) $(($res_index, $res_ident))*);
    };

    (@solve $index:expr, , $(($res_index:expr, $res_ident:ident))*) => {
        const DAY_COUNT: u8 = $index;

        fn solve_and_print(day: u8) {
            match day {
                $(
                    i if i == $res_index => print_output($res_ident::solve(
                        include_str!(concat!("../input/", stringify!($res_ident), ".txt"))
                    )),
                )+
                _ => unreachable!(),
            }
        }
    };
}

declare_days!(
    day_01
    day_02
    day_03
    day_04
    day_05
);

use clap::Parser;
use std::fmt::Display;

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
    let sep = if output.contains('\n') { '\n' } else { ' ' };
    println!("{sep}{output}");
}

fn print_output<P1: Display, P2: Display>(output: (P1, P2)) {
    print_part(1, output.0.to_string());
    print_part(2, output.1.to_string());
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
