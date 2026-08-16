use anyhow::Result;
use aoc::{launch_solver, Solver};

struct Day4Part1Solver();

impl Solver for Day4Part1Solver {
    fn solve(input_data: String) -> u32 {
        let data_lines = input_data.lines();
        let mut sum = 0;

        let parse_as_numbers = |part: &str| -> Vec<u32> {
            part.split_whitespace()
                .flat_map(|x| x.parse::<u32>())
                .collect()
        };

        for line in data_lines {
            let mut line_sum = 0;

            let (left, right) = {
                let mut parts = line.split(": ").nth(1).unwrap().split(" | ");
                (
                    parse_as_numbers(parts.next().unwrap()),
                    parse_as_numbers(parts.next().unwrap()),
                )
            };

            for number in right {
                if left.contains(&number) {
                    line_sum = if line_sum == 0 { 1 } else { line_sum * 2 };
                }
            }

            sum += line_sum
        }
        sum
    }
}
fn main() -> Result<()> {
    launch_solver::<Day4Part1Solver>()
}
