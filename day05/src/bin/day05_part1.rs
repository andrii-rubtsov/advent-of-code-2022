/*! See https://adventofcode.com/2022/day/5 */

use std::io::{BufRead, BufReader, Read};

use day05::{Command, CrateStacks};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn process_and_peek_top_letters(reader: impl Read) -> Result<String, Box<dyn std::error::Error>> {
    let mut stack_lines = vec![];
    let mut command_lines = vec![];
    let mut separator_found = false;
    for line_maybe in BufReader::new(reader).lines() {
        let line = line_maybe?;
        if line.is_empty() {
            separator_found = true;
        } else if separator_found {
            command_lines.push(line);
        } else {
            stack_lines.push(line);
        }
    }

    let mut stack = CrateStacks::from_text_repr(stack_lines);
    let commands: Vec<Command> = command_lines
        .iter()
        .map(|line| line.as_str().into())
        .collect();

    for command in commands {
        stack.apply(&command);
    }

    Ok(stack
        .peek_top_letters()
        .iter()
        .map(|&e| e.unwrap_or(&('-')))
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let top_letters = process_and_peek_top_letters(asset.data.as_ref())?;
    println!("Top letters are: {top_letters}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(
            process_and_peek_top_letters(asset.data.as_ref()).unwrap(),
            String::from("CMZ")
        );
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(
            process_and_peek_top_letters(asset.data.as_ref()).unwrap(),
            String::from("RTGWZTHLD")
        );
    }
}
