/*! See https://adventofcode.com/2022/day/3 */

#![allow(non_upper_case_globals)]

use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
};

use day03::get_priority;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn find_common(rucksack_str: &str) -> char {
    let (first, second) = rucksack_str.split_at(rucksack_str.len() / 2);
    let first_set: HashSet<char> = HashSet::from_iter(first.chars());
    let second_set: HashSet<char> = HashSet::from_iter(second.chars());
    let common: Vec<_> = first_set.intersection(&second_set).collect();
    if common.len() != 1 {
        panic!(
            "Expected exactly one common char in '{}', but found '{:?}'",
            rucksack_str, common
        )
    }
    *common[0]
}

fn get_priorities_sum(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let sum: usize = BufReader::new(reader)
        .lines()
        .map(|line_maybe| find_common(line_maybe.unwrap().as_str()))
        .map(get_priority)
        .sum();
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let sum: usize = get_priorities_sum(asset.data.as_ref())?;
    println!("Total sum is {sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(get_priorities_sum(asset.data.as_ref()).unwrap(), 157);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("part1_input.txt").unwrap();
        assert_eq!(get_priorities_sum(asset.data.as_ref()).unwrap(), 8039);
    }
}
