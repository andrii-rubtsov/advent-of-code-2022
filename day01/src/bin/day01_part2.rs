/*! See https://adventofcode.com/2022/day/1 */

use itertools::Itertools;
use rust_embed::RustEmbed;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{BufRead, BufReader, Read},
};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn sum_calories_by_top(reader: impl Read, top_num: usize) -> usize {
    let group_sums = BufReader::new(reader)
        .lines()
        .map(|line| line.unwrap().parse::<usize>().ok())
        .batching(|it| {
            it.take_while(|it| it.is_some())
                .map(|maybe_num| maybe_num.unwrap())
                .sum1()
        });

    // Reverse allows to efficiently remove the smallest value form the "top+1" - sized max heap:
    // Even though it is a "max" heap, the needed for `Reverse` comes from the fact that 
    // heap only provides an efficient (O(1)) way to remove the "largest" value via `pop`.
    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::with_capacity(top_num + 1);
    for group in group_sums {
        heap.push(Reverse(group));
        if heap.len() > top_num {
            heap.pop();
        }
    }
    heap.iter().map(|reversed| reversed.0).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let sum_calories_by_top = sum_calories_by_top(asset.data.as_ref(), 3);
    println!("Total calories by top 3: {}", sum_calories_by_top);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(sum_calories_by_top(asset.data.as_ref(), 3), 45000);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(sum_calories_by_top(asset.data.as_ref(), 3), 212117);
    }
}
