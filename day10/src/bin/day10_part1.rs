/*! See https://adventofcode.com/2022/day/10 */

use rust_embed::RustEmbed;
use std::io::Read;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn sum_of_signal_strength(read: impl Read) -> Result<i32, Box<dyn std::error::Error>> {
    let x = day10::x_register_values(read)?;
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
