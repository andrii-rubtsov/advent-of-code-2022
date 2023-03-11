/*! See https://adventofcode.com/2022/day/6 */

use day6::detect_start_of_unique_window;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn detect_message_start() -> Result<usize, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("input.txt").unwrap();
    let input_string = std::str::from_utf8(input_resource.data.as_ref())?;
    let number = detect_start_of_unique_window(input_string, 14)?;
    Ok(number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message_start = detect_message_start()?;
    println!("Number of characters before message start: {message_start}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(detect_message_start().unwrap(), 3425);
    }
}
