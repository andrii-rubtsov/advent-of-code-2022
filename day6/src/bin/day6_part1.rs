/*! See https://adventofcode.com/2022/day/6 */

use day6::detect_start_of_unique_window;

fn detect_packet_start() -> Result<usize, Box<dyn std::error::Error>> {
    let input_string = std::fs::read_to_string(utils::find_empirically("day6/input.txt"))?;
    let number = detect_start_of_unique_window(&input_string, 4)?;
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
