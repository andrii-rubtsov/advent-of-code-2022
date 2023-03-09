use itertools::{Chunk, Itertools};
use std::{collections::HashSet, str::Lines};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_str = include_str!("../../part2_input.txt");
    let total_sum: u32 = input_str
        .lines()
        .chunks(3)
        .into_iter()
        .map(|block| find_common_char(block).unwrap())
        .map(day3::get_priority)
        .sum();

    println!("Total sum is {total_sum}");
    Ok(())
}
