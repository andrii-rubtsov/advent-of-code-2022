/*! https://adventofcode.com/2022/day/3
   Part 1
*/

use std::collections::HashSet;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_str = include_str!("../../part1_input.txt");
    let sum: u32 = input_str
        .lines()
        .map(find_common)
        .map(day3::get_priority)
        .sum();

    println!("Total sum is {sum}");
    Ok(())
}
