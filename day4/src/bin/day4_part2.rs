/*! See https://adventofcode.com/2022/day/4 */

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn count_overlaps() -> Result<usize, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("input.txt").unwrap();
    let input = std::str::from_utf8(input_resource.data.as_ref())?;
    let overlaped_count = input
        .lines()
        .map(|line| {
            let mut parts: Vec<day4::Range> = line.split(',').map(|s| s.into()).collect();
            if parts.len() != 2 {
                panic!("Expected to have exactly 2 ranges, but found: {:?}", parts);
            }
            let second = parts.pop().unwrap();
            let first = parts.pop().unwrap();

            (first, second)
        })
        .filter(|(range_one, range_two)| {
            range_one.overlaps(range_two) || range_two.overlaps(range_one)
        })
        .count();
    Ok(overlaped_count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let overlaps_count = count_overlaps()?;
    println!("Overlaps count: {overlaps_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(count_overlaps().unwrap(), 938);
    }
}
