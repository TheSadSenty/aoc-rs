use anyhow::Result;
use aoc::{launch_solver, Solver};

struct Day2Part2Solver();

impl Solver for Day2Part2Solver {
    fn solve(input_data: String) -> u32 {
        input_data
            .lines()
            .map(|line| {
                line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split([';', ','])
                    .fold([0u32; 3], |mut acc, cube| {
                        let (num, color) = cube.trim().split_once(' ').unwrap();
                        let num = num.parse().unwrap();
                        let color_index = match color {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => panic!("Got unexpected color"),
                        };
                        acc[color_index] = acc[color_index].max(num);
                        acc
                    })
                    .into_iter()
                    .product::<u32>()
            })
            .sum()
    }
}
fn main() -> Result<()> {
    launch_solver::<Day2Part2Solver>()
}
