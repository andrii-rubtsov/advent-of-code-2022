/*! See https://adventofcode.com/2022/day/4 */

#![allow(non_upper_case_globals)]

use std::io::{BufRead, BufReader, Read};

use day04::Range;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn count_fully_contained_ranges(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let fully_contained_count = BufReader::new(reader)
        .lines()
        .map(|line_maybe| line_maybe.unwrap())
        .map(|line| {
            let mut parts: Vec<Range> = line.split(',').map(|s| s.into()).collect();
            if parts.len() != 2 {
                panic!("Expected to have exactly 2 ranges, but found: {:?}", parts);
            }
            let second = parts.pop().unwrap();
            let first = parts.pop().unwrap();

            (first, second)
        })
        .filter(|(range_one, range_two)| {
            range_one.contains_fully(range_two) || range_two.contains_fully(range_one)
        })
        .count();
    Ok(fully_contained_count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let fully_contained_count = count_fully_contained_ranges(asset.data.as_ref())?;
    println!("Fully contained count: {fully_contained_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(
            count_fully_contained_ranges(asset.data.as_ref()).unwrap(),
            2
        );
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(
            count_fully_contained_ranges(asset.data.as_ref()).unwrap(),
            657
        );
    }
}
