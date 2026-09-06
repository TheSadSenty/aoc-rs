use anyhow::Result;
use aoc::{launch_solver, Solver};

const DIGITS: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

struct Day1Part2Solver();

impl Solver for Day1Part2Solver {
    fn solve(input_data: String) -> u32 {
        input_data
            .lines()
            .map(|line| {
                let mut digits = (0..line.len()).flat_map(|i| {
                    let rest = &line[i..];
                    DIGITS.iter().filter_map(|(digit_as_str, digit)| {
                        rest.starts_with(digit_as_str).then_some(digit)
                    })
                });

                let first_digit = digits.next().unwrap_or(&0);

                10 * first_digit + digits.next_back().unwrap_or(first_digit)
            })
            .sum::<u32>()
    }
}
fn main() -> Result<()> {
    launch_solver::<Day1Part2Solver>()
}
