/*! See https://adventofcode.com/2022/day/2 */

use std::io::{BufRead, BufReader, Read};

use day02::Round;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn get_total_points(reader: impl Read) -> usize {
    BufReader::new(reader)
        .lines()
        .map(|line_maybe| line_maybe.unwrap())
        .map(|round_str| round_str.parse::<Round>().unwrap())
        .map(|round| round.total_points())
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let total_points = get_total_points(asset.data.as_ref());
    println!("Total points: {total_points}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(get_total_points(asset.data.as_ref()), 15);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(get_total_points(asset.data.as_ref()), 13526);
    }
}
