/*! See https://adventofcode.com/2022/day/11 */

use day11::{calculate_monkey_business, parse_all_monkeys, process_round};
use rust_embed::RustEmbed;
use std::io::Read;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn process_rounds(reader: impl Read) -> u128 {
    let mut monkeys = parse_all_monkeys(reader);

    for _ in 0..20 {
        process_round(&mut monkeys, true, None);
    }

    calculate_monkey_business(&mut monkeys)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let monkey_business = process_rounds(asset.data.as_ref());
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
        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let monkey_business = process_rounds(asset.data.as_ref());
        assert_eq!(monkey_business, 72884);
    }
}
