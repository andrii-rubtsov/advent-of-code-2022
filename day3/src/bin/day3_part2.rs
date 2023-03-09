use itertools::{Chunk, Itertools};
use std::{collections::HashSet, str::Lines};

#[derive(Debug)]
struct CommonCharError {
    #[allow(unused)]
    msg: String,
}

fn find_common_char(mut block: Chunk<Lines>) -> Result<char, CommonCharError> {
    let f = block.next().unwrap().chars();
    let mut inter: HashSet<char> = HashSet::from_iter(f);
    for str in block {
        let a: HashSet<char> = HashSet::from_iter(str.chars());
        inter.retain(|c| a.contains(c));
    }
    if inter.len() != 1 {
        Result::Err(CommonCharError {
            msg: format!("Expected exactly 1 common element, but found: {:?}", inter),
        })
    } else {
        Ok(inter.into_iter().next().unwrap())
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
