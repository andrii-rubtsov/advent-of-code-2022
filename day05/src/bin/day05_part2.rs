/*! See https://adventofcode.com/2022/day/5 */

use day05::{Command, CrateStacks};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn process_preserve_order_and_peek_top_letters() -> Result<String, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("input.txt").unwrap();
    let input = std::str::from_utf8(input_resource.data.as_ref())?;

    let mut stack_lines = vec![];
    let mut command_lines = vec![];
    let mut separator_found = false;
    for line in input.lines() {
        if line.is_empty() {
            separator_found = true;
        } else if separator_found {
            command_lines.push(line);
        } else {
            stack_lines.push(line);
        }
    }

    let mut stack = CrateStacks::from_text_repr(stack_lines);
    let commands: Vec<Command> = command_lines.iter().map(|&line| line.into()).collect();

    for command in commands {
        stack.apply_preserve_order(&command);
    }

    Ok(stack
        .peek_top_letters()
        .iter()
        .map(|&e| e.unwrap_or(&('-')))
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let top_letters = process_preserve_order_and_peek_top_letters()?;
    println!("Top letters are: {top_letters}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(
            process_preserve_order_and_peek_top_letters().unwrap(),
            String::from("STHGRZZFR")
        );
    }
}
