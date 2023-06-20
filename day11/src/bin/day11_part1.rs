/*! See https://adventofcode.com/2022/day/11 */

use day11::Monkey;
use rust_embed::RustEmbed;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn parse_all_monkeys(reader: impl Read) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut buffer = String::new();
    for input_line in BufReader::new(reader).lines() {
        let line = input_line.unwrap();
        if line.is_empty() {
            if !buffer.is_empty() {
                monkeys.push(Monkey::parse(&buffer));
            }
            buffer = String::new();
        } else {
            buffer.push_str(&line);
            buffer.push('\n');
        }
    }
    if !buffer.is_empty() {
        monkeys.push(Monkey::parse(&buffer));
    }
    monkeys
}

fn process_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let mut new_owners: HashMap<usize, Vec<u32>> = HashMap::new();

        let mut monkey = &mut monkeys[i];
        for stress_level in &monkey.items {
            let new_stress_level = monkey.operation.apply(*stress_level) / 3;
            let next_monkey_idx = monkey.next_monkey.next_monkey_index(new_stress_level);
            new_owners
                .entry(next_monkey_idx)
                .or_default()
                .push(new_stress_level);
        }
        monkey.total_inspections += monkey.items.len();
        monkey.items.clear();

        for (idx, mut new_items) in new_owners {
            monkeys[idx].items.append(&mut new_items);
        }
    }
}

fn monkey_business(reader: impl Read) -> usize {
    let mut monkeys = parse_all_monkeys(reader);

    for _ in 0..20 {
        process_round(&mut monkeys);
    }

    monkeys.sort_by_key(|m| -(m.total_inspections as i32));
    monkeys
        .iter()
        .take(2)
        .map(|m| m.total_inspections)
        .reduce(|acc, e| acc * e)
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let reader = asset.data.as_ref();

    let monkey_business = monkey_business(reader);

    println!("Monkey business: {monkey_business}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let monkey_business = monkey_business(asset.data.as_ref());
        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let monkey_business = monkey_business(asset.data.as_ref());
        assert_eq!(monkey_business, 72884);
    }
}
