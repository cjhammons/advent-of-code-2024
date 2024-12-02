mod days;

use clap::Parser;

#[derive(Parser)]
#[command(name = "advent-of-code")]
#[command(about = "Solutions for Advent of Code")]
struct Args {
    #[arg(value_parser = clap::value_parser!(u32).range(1..=25))]
    day: u32,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => days::day01::run(),
        2 => days::day02::run(),
        _ => println!("Day {} not implemented", args.day)
    };
}
