/*! See https://adventofcode.com/2022/day/6 */

use day06::detect_start_of_unique_window;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn detect_packet_start() -> Result<usize, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("input.txt").unwrap();
    let input_string = std::str::from_utf8(input_resource.data.as_ref())?;
    let number = detect_start_of_unique_window(input_string, 4)?;
    Ok(number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message_start = detect_packet_start()?;
    println!("Number of characters before packet start: {message_start}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(detect_packet_start().unwrap(), 1833);
    }
}
