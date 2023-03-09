/*! See https://adventofcode.com/2022/day/2
 * Part 1
 */

use day2::Round;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_str = include_str!("../../input.txt");
    let total_points: u32 = input_str
        .lines()
        .map(|round_str| -> Round { round_str.parse().unwrap() })
        .map(|round| round.total_points())
        .sum();
    println!("Total points: {total_points}");
    Ok(())
}
