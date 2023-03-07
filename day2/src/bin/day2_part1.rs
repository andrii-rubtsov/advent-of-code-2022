/*! See https://adventofcode.com/2022/day/2
 * Part 1
 */

use day2::Round;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_points: u32 = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|round_str| -> Round { round_str.parse().unwrap() })
        .map(|round| round.total_points())
        .sum();
    println!("Total points: {total_points}");
    Ok(())
}
