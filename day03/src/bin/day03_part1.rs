/*! https://adventofcode.com/2022/day/3
   Part 1
*/

use std::collections::HashSet;

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

fn get_priorities_sum() -> Result<u32, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("part1_input.txt").unwrap();
    let input = std::str::from_utf8(input_resource.data.as_ref())?;
    let sum: u32 = input
        .lines()
        .map(find_common)
        .map(day03::get_priority)
        .sum();
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sum: u32 = get_priorities_sum()?;
    println!("Total sum is {sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(get_priorities_sum().unwrap(), 8039);
    }
}
