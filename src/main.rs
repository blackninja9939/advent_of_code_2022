use color_eyre::eyre::{self, Result};

use clap::Parser;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Which days advent of code to run
    #[arg( short, long, value_parser = clap::value_parser!(i32).range(1..=24) )]
    day: i32,

    /// Which puzzle from the day to run
    #[arg( short, long, value_parser = clap::value_parser!(i32).range(1..=2 ), default_value_t = 1 )]
    puzzle: i32,
}

fn run_day(day: i32, puzzle: i32, data: String) -> Result<()> {
    match (day, puzzle) {
        (1, 1) => {
            day_1::puzzle_1(data);
        }
        (1, 2) => {
            day_1::puzzle_2(data);
        }
        (2, 1) => {
            day_2::puzzle_1(data);
        }
        (2, 2) => {
            day_2::puzzle_2(data);
        }
        (3, 1) => {
            day_3::puzzle_1(data);
        }
        (3, 2) => {
            day_3::puzzle_2(data);
        }
        (4, 1) => {
            day_4::puzzle_1(data);
        }
        (4, 2) => {
            day_4::puzzle_2(data);
        }
        (5, 1) => {
            day_5::puzzle_1(data);
        }
        (5, 2) => {
            day_5::puzzle_2(data);
        }
        (_, _) => return eyre!("Day {} has not been finished yet", day),
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    // let arguments = Args::parse();
    let arguments = Args { day: 5, puzzle: 1 };
    let file_name = format!("./data/day_{}.txt", arguments.day);
    let data = std::fs::read_to_string(file_name)?;
    run_day(arguments.day, arguments.puzzle, data)
}
