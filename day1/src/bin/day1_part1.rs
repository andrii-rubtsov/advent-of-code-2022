/*! See https://adventofcode.com/2022/day/1 */

fn get_max_calories() -> Result<u32, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(utils::find_empirically("day1/input.txt"))?;
    Ok(input
        .split("\n\n")
        .map(|block| block.lines().map(|s| s.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_elf_calories: u32 = get_max_calories()?;
    println!("Max calories per elf: {}", max_elf_calories);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(get_max_calories().unwrap(), 72511);
    }
}
