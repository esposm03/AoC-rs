#![allow(non_snake_case)]

use clap::Parser;
use std::io;
use std::io::Read;

mod solutions_2019;
mod solutions_2020;

#[derive(Parser)]
#[clap(
    version = "2020",
    author = "esposm03 <36164633+esposm03@users.noreply.github.com>"
)]
struct Invocation {
    /// The AoC year (default: 2020)
    #[clap(short, long, default_value = "2020")]
    year: usize,
    /// The problem number
    problem: usize,
    /// The problem part (1 or 2)
    part: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SolutionType {
    Int(i64),
    OptionInt(Option<i64>),
    String(String),
}

impl From<i64> for SolutionType {
    fn from(i: i64) -> Self {
        Self::Int(i)
    }
}
impl From<usize> for SolutionType {
    fn from(i: usize) -> Self {
        Self::Int(i.try_into().unwrap())
    }
}
impl From<Option<i64>> for SolutionType {
    fn from(i: Option<i64>) -> Self {
        Self::OptionInt(i)
    }
}
impl From<&'_ str> for SolutionType {
    fn from(i: &'_ str) -> Self {
        Self::String(i.to_string())
    }
}

#[macro_export]
macro_rules! declare_solutions {
    () => {
        pub const SOLUTIONS: [for<'r> fn(&'r str) -> SolutionType; 50] = [
            day1,
            day1_part2,
            day2,
            day2_part2,
            day3,
            day3_part2,
            day4,
            day4_part2,
            day5,
            day5_part2,
            day6,
            day6_part2,
            day7,
            day7_part2,
            day8,
            day8_part2,
            day9,
            day9_part2,
            day10,
            day10_part2,
            day11,
            day11_part2,
            day12,
            day12_part2,
            day13,
            day13_part2,
            day14,
            day14_part2,
            day15,
            day15_part2,
            day16,
            day16_part2,
            day17,
            day17_part2,
            day18,
            day18_part2,
            day19,
            day19_part2,
            day20,
            day20_part2,
            day21,
            day21_part2,
            day22,
            day22_part2,
            day23,
            day23_part2,
            day24,
            day24_part2,
            day25,
            day25_part2,
        ];
    };
}

fn main() {
    let invocation = Invocation::parse();
    let number = invocation.problem * 2 + invocation.part - 3;

    if invocation.problem > 25 || invocation.problem < 1 {
        println!("Invalid problem :(");
        return;
    }
    if invocation.part != 1 && invocation.part != 2 {
        println!("Invalid part :(");
        return;
    }

    println!("Now paste the input: ");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("IO error");

    println!(
        "{:?}",
        match invocation.year {
            2019 => solutions_2019::SOLUTIONS[number](&input),
            2020 => solutions_2020::SOLUTIONS[number](&input),
            _ => "Invalid year :(".into(),
        }
    );
}
