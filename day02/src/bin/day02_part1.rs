/*! See https://adventofcode.com/2022/day/2
 * Part 1
 */

use day02::Round;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn get_total_points() -> Result<u32, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("input.txt").unwrap();
    let input = std::str::from_utf8(input_resource.data.as_ref())?;
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
