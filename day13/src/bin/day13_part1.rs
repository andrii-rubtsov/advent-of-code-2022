/*! See https://adventofcode.com/2022/day/13 */

#![allow(non_upper_case_globals)]

use day13::{parse_pairs, PacketPair};
use rust_embed::RustEmbed;
use std::io::Read;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn sum_right_ordered_indices(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let sum = parse_pairs(reader)?
        .into_iter()
        .enumerate()
        .filter_map(|(idx, pair): (usize, PacketPair)| {
            Some(idx + 1).filter(|_| pair.is_right_order())
        })
        .sum();
    Ok(sum)
}

fn main() {
    let asset = Asset::get("input.txt").unwrap();
    let right_ordered_sum = sum_right_ordered_indices(asset.data.as_ref()).unwrap();
    println!("Sum of right ordered indices: {right_ordered_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let right_ordered_sum = sum_right_ordered_indices(asset.data.as_ref()).unwrap();
        assert_eq!(right_ordered_sum, 13);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let right_ordered_sum = sum_right_ordered_indices(asset.data.as_ref()).unwrap();
        assert_eq!(right_ordered_sum, 5196);
    }
}
