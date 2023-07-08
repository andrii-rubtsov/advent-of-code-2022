/*! See https://adventofcode.com/2022/day/4 */

use std::io::{BufRead, BufReader, Read};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn count_overlaps(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let overlaped_count = BufReader::new(reader)
        .lines()
        .map(|line| {
            let mut parts: Vec<day04::Range> = line.unwrap().split(',').map(|s| s.into()).collect();
            if parts.len() != 2 {
                panic!("Expected to have exactly 2 ranges, but found: {:?}", parts);
            }
            let second = parts.pop().unwrap();
            let first = parts.pop().unwrap();

            (first, second)
        })
        .filter(|(range_one, range_two)| {
            range_one.overlaps(range_two) || range_two.overlaps(range_one)
        })
        .count();
    Ok(overlaped_count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let overlaps_count = count_overlaps(asset.data.as_ref())?;
    println!("Overlaps count: {overlaps_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(count_overlaps(asset.data.as_ref()).unwrap(), 4);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(count_overlaps(asset.data.as_ref()).unwrap(), 938);
    }
}
