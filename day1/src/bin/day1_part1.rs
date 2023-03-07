/*! See https://adventofcode.com/2022/day/1 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_elf_calories: u32 = std::fs::read_to_string("input.txt")?
        .split("\n\n")
        .map(|block| block.lines().map(|s| s.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap();

    println!("Max calories per elf: {}", max_elf_calories);
    Ok(())
}
