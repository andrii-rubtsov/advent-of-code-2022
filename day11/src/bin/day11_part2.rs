/*! See https://adventofcode.com/2022/day/11 */

use day11::{calculate_monkey_business, parse_all_monkeys, process_round};
use rust_embed::RustEmbed;
use std::io::Read;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn process_rounds(reader: impl Read) -> u128 {
    let mut monkeys = parse_all_monkeys(reader);

    let common_divisor = monkeys
        .iter()
        .map(|m| m.divisible_by_test.divisor())
        .reduce(|acc, e| acc * e)
        .unwrap();

    for _ in 1..10001 {
        process_round(&mut monkeys, false, Some(common_divisor));
    }

    calculate_monkey_business(&mut monkeys)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let reader = asset.data.as_ref();

    let monkey_business = process_rounds(reader);

    println!("Monkey business: {monkey_business}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let monkey_business = process_rounds(asset.data.as_ref());
        assert_eq!(monkey_business, 2713310158);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let monkey_business = process_rounds(asset.data.as_ref());
        assert_eq!(monkey_business, 15310845153);
    }
}
