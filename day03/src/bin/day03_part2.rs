/*! See https://adventofcode.com/2022/day/3 */

#![allow(non_upper_case_globals)]

use itertools::{Chunk, Itertools};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
struct CommonCharError {
    #[allow(unused)]
    msg: String,
}

fn find_common_char<T: Iterator<Item = String>>(block: Chunk<T>) -> Result<char, CommonCharError> {
    let intersection = block
        .map(|line| HashSet::from_iter(line.chars()))
        .reduce(|mut acc_set: HashSet<_>, set: HashSet<_>| {
            acc_set.retain(|item| set.contains(item));
            acc_set
        })
        .unwrap();

    if intersection.len() != 1 {
        Result::Err(CommonCharError {
            msg: format!(
                "Expected exactly 1 common element, but found: {:?}",
                intersection
            ),
        })
    } else {
        Ok(intersection.into_iter().next().unwrap())
    }
}

fn find_common_and_sum_priorities(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let total_sum: usize = BufReader::new(reader)
        .lines()
        .map(|s| s.unwrap())
        .chunks(3)
        .into_iter()
        .map(|block| find_common_char(block).unwrap())
        .map(day03::get_priority)
        .sum();
    Ok(total_sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("part2_input.txt").unwrap();
    let total_sum = find_common_and_sum_priorities(asset.data.as_ref())?;
    println!("Total sum is {total_sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(
            find_common_and_sum_priorities(asset.data.as_ref()).unwrap(),
            70
        );
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("part2_input.txt").unwrap();
        assert_eq!(
            find_common_and_sum_priorities(asset.data.as_ref()).unwrap(),
            2510
        );
    }
}
