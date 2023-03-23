/*! See https://adventofcode.com/2022/day/10 */

use lazy_static::lazy_static;
use regex::Regex;
use rust_embed::RustEmbed;
use std::io::{BufRead, BufReader, Read};

lazy_static! {
    static ref CMD_NOOP: Regex = Regex::new(r"noop").unwrap();
    static ref CMD_ADDX: Regex = Regex::new(r"addx (?P<value>[-]?\d+)").unwrap();
}

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn sum_of_signal_strength(read: impl Read) -> Result<i32, Box<dyn std::error::Error>> {
    let mut x = Vec::with_capacity(300);
    x.push(1); // x[0] and x[1] - initial value set to `1`
    x.push(1);

    for line in BufReader::new(read).lines() {
        let cmd_line = &line?;
        if CMD_NOOP.is_match(cmd_line) {
            x.push(*x.last().unwrap());
        } else if CMD_ADDX.is_match(cmd_line) {
            let captures = CMD_ADDX.captures_iter(cmd_line).next().unwrap();
            let value: i32 = captures.name("value").unwrap().as_str().parse().unwrap();
            x.push(*x.last().unwrap());
            x.push(*x.last().unwrap() + value);
        } else {
            unreachable!("Unknown command: {cmd_line}")
        }
    }

    Ok((20..=220).step_by(40).map(|i| i as i32 * x[i]).sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let sum_of_signal_strength = sum_of_signal_strength(asset.data.as_ref())?;
    println!("Total unique tail locations: {sum_of_signal_strength}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let sum_of_signal_strength = sum_of_signal_strength(asset.data.as_ref()).unwrap();
        assert_eq!(sum_of_signal_strength, 13140);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let total_unique_tail_locations = sum_of_signal_strength(asset.data.as_ref()).unwrap();
        assert_eq!(total_unique_tail_locations, 14060);
    }
}
