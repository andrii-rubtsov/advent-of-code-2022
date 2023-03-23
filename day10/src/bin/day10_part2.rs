/*! See https://adventofcode.com/2022/day/10 */

use rust_embed::RustEmbed;
use std::io::Read;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn process_crt(read: impl Read) -> Result<String, Box<dyn std::error::Error>> {
    let register_at_cycle = day10::x_register_values(read)?;

    let mut output = String::with_capacity(std::cmp::max(256, (40 + 1) * 6));
    for (cycle, stripe) in register_at_cycle.iter().enumerate().take(240 + 1).skip(1) {
        if i32::abs(((cycle - 1) % 40) as i32 - stripe) < 2 {
            output.push('#');
        } else {
            output.push('.');
        }
        if cycle % 40 == 0 {
            output.push('\n');
        }
    }
    Ok(output)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_asset = Asset::get("input.txt").unwrap();
    let crt_output = process_crt(input_asset.data.as_ref())?;

    println!("Crt output:");
    println!("{crt_output}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let expected_test_asset = Asset::get("expected_test_output.txt").unwrap();
        let expected_output = std::str::from_utf8(expected_test_asset.data.as_ref()).unwrap();

        let asset = Asset::get("test_input.txt").unwrap();
        let crt_output = process_crt(asset.data.as_ref()).unwrap();
        assert_eq!(&crt_output, expected_output);
    }

    #[test]
    fn actual_input() {
        let expected_test_asset = Asset::get("expected_output.txt").unwrap();
        let expected_output = std::str::from_utf8(expected_test_asset.data.as_ref()).unwrap();

        let asset = Asset::get("input.txt").unwrap();
        let crt_output = process_crt(asset.data.as_ref()).unwrap();
        assert_eq!(&crt_output, expected_output);
    }
}
