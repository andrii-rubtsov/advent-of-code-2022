/*! See https://adventofcode.com/2022/day/1 */
use itertools::Itertools;

use std::io::{BufRead, BufReader, Read};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn get_max_calories(reader: impl Read) -> usize {
    BufReader::new(reader)
        .lines()
        .map(|line| line.unwrap().parse::<usize>().ok())
        .batching(|it| {
            it.take_while(|it| it.is_some())
                .map(|maybe_num: Option<usize>| maybe_num.unwrap())
                .sum1()
        })
        .max()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let max_elf_calories: usize = get_max_calories(asset.data.as_ref());
    println!("Max calories per elf: {}", max_elf_calories);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(get_max_calories(asset.data.as_ref()), 24000);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(get_max_calories(asset.data.as_ref()), 72511);
    }
}
