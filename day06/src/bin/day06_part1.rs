/*! See https://adventofcode.com/2022/day/6 */

use std::io::Read;

use day06::detect_start_of_unique_window;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn detect_packet_start(mut reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let mut input_string = String::new();
    reader.read_to_string(&mut input_string).unwrap();
    let number = detect_start_of_unique_window(input_string.as_str(), 4)?;
    Ok(number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("test_input.txt").unwrap();
    let message_start = detect_packet_start(asset.data.as_ref())?;
    println!("Number of characters before packet start: {message_start}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(detect_packet_start(asset.data.as_ref()).unwrap(), 7);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(detect_packet_start(asset.data.as_ref()).unwrap(), 1833);
    }
}
