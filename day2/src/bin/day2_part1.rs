/*! See https://adventofcode.com/2022/day/2
 * Part 1
 */

use day2::Round;

fn get_total_points() -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(utils::find_empirically("day2/input.txt"))?;
    Ok(input
        .lines()
        .map(|round_str| -> Round { round_str.parse().unwrap() })
        .map(|round| round.total_points())
        .sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_points = get_total_points()?;
    println!("Total points: {total_points}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(get_total_points().unwrap(), 13526);
    }
}
