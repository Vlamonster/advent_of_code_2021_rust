use advent_of_code_2021_rust::{d01, d02};
use clap::Parser;
use std::fs;

type Solution = fn(&str) -> String;

const SOLUTIONS: [[Solution; 2]; 2] = [[d01::p1, d01::p2], [d02::p1, d02::p2]];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run. None will run all problems.
    day: Option<usize>,

    /// Part to run. None will run both parts.
    part: Option<usize>,

    /// Path to input file to use. None will use my input file.
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.day.is_none() {
        for day in 1..=SOLUTIONS.len() {
            let input = fs::read_to_string(format!("./inputs/d{:02}.txt", day)).unwrap();
            println!("d{day:02}a: {}", SOLUTIONS[day - 1][0](input.as_str()));
            println!("d{day:02}b: {}", SOLUTIONS[day - 1][1](input.as_str()));
        }
        return;
    }

    let day = args.day.unwrap();
    let input = match args.input {
        Some(path) => fs::read_to_string(path).unwrap(),
        None => fs::read_to_string(format!("./inputs/d{:02}.txt", day)).unwrap(),
    };

    if args.part.is_none() {
        println!("d{day:02}a: {}", SOLUTIONS[day - 1][0](input.as_str()));
        println!("d{day:02}a: {}", SOLUTIONS[day - 1][1](input.as_str()));
        return;
    }

    let part = args.part.unwrap();

    println!(
        "d{day:02}{}: {}",
        if part == 1 { 'a' } else { 'b' },
        SOLUTIONS[day - 1][part - 1](input.as_str())
    );
}
