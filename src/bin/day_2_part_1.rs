use anyhow::Result;
use aoc::{launch_solver, Solver};

const MAX_RED_CUBES_NUM: u32 = 12;
const MAX_GREEN_CUBES_NUM: u32 = 13;
const MAX_BLUE_CUBES_NUM: u32 = 14;

struct Day2Part1Solver();

impl Solver for Day2Part1Solver {
    fn solve(input_data: String) -> u32 {
        input_data
            .lines()
            .enumerate()
            .filter(|(_, line)| {
                line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split("; ")
                    .all(|cube_set| {
                        cube_set.split(", ").all(|cubes| {
                            let (num, color) = cubes.split_once(' ').unwrap();
                            let num = num.parse::<u32>().unwrap();
                            match color {
                                "red" => num <= MAX_RED_CUBES_NUM,
                                "green" => num <= MAX_GREEN_CUBES_NUM,
                                "blue" => num <= MAX_BLUE_CUBES_NUM,
                                _ => panic!("Got unexpected color"),
                            }
                        })
                    })
            })
            .map(|(index, _)| index as u32 + 1)
            .sum()
    }
}
fn main() -> Result<()> {
    launch_solver::<Day2Part1Solver>()
}
