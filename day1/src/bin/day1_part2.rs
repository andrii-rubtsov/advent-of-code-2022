/*! See https://adventofcode.com/2022/day/1 */

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn sum_calories_by_top(top: usize) -> Result<u32, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("input.txt").unwrap();
    let input = std::str::from_utf8(input_resource.data.as_ref())?;
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|block| block.lines().map(|s| s.parse::<u32>().unwrap()).sum())
        .collect();

    calories.sort_by(|v1, v2| v2.cmp(v1));

    Ok(calories.iter().take(top).sum::<u32>())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sum_calories_by_top: u32 = sum_calories_by_top(3)?;
    println!("Total calories by top 3: {}", sum_calories_by_top);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(sum_calories_by_top(3).unwrap(), 212117);
    }
}
