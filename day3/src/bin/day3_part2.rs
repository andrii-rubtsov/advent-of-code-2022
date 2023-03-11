use itertools::{Chunk, Itertools};
use std::{collections::HashSet, str::Lines};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
struct CommonCharError {
    #[allow(unused)]
    msg: String,
}

fn find_common_char(block: Chunk<Lines>) -> Result<char, CommonCharError> {
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

fn find_common_and_sum_priorities() -> Result<u32, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("part2_input.txt").unwrap();
    let input = std::str::from_utf8(input_resource.data.as_ref())?;
    let total_sum: u32 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|block| find_common_char(block).unwrap())
        .map(day3::get_priority)
        .sum();
    Ok(total_sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_sum = find_common_and_sum_priorities()?;
    println!("Total sum is {total_sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(find_common_and_sum_priorities().unwrap(), 2510);
    }
}
