/*! See https://adventofcode.com/2022/day/4 */

fn count_fully_contained_ranges() -> Result<usize, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(utils::find_empirically("day4/input.txt"))?;
    let fully_contained_count = input
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
            range_one.contains_fully(range_two) || range_two.contains_fully(range_one)
        })
        .count();
    Ok(fully_contained_count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fully_contained_count = count_fully_contained_ranges()?;
    println!("Fully contained count: {fully_contained_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(count_fully_contained_ranges().unwrap(), 657);
    }
}
