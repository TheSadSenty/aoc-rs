use anyhow::Result;
use aoc::{launch_solver, Solver};

struct Day1Part1Solver();

impl Solver for Day1Part1Solver {
    fn solve(input_data: String) -> u32 {
        input_data
            .lines()
            .map(|line| {
                let mut digits = line.chars().flat_map(|c| c.to_digit(10));

                let first_digit = digits.next().unwrap_or(0);

                10 * first_digit + digits.next_back().unwrap_or(first_digit)
            })
            .sum::<u32>()
    }
}
fn main() -> Result<()> {
    launch_solver::<Day1Part1Solver>()
}
