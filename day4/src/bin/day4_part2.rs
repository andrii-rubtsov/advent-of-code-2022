/*! See https://adventofcode.com/2022/day/4 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_str = include_str!("../../input.txt");
    let fully_contained_count = input_str
        .lines()
        .map(|line| {
            let mut parts: Vec<day4::Range> = line.split(",").map(|s| s.into()).collect();
            if parts.len() != 2 {
                panic!("Expected to have exactly 2 ranges, but found: {:?}", parts);
            }
            let second = parts.pop().unwrap();
            let first = parts.pop().unwrap();

            (first, second)
        })
        .filter(|(range_one, range_two)| {
            range_one.overlaps(&range_two) || range_two.overlaps(&range_one)
        })
        .count();

    println!("Fully contained count: {fully_contained_count}");
    Ok(())
}
