/*! See https://adventofcode.com/2022/day/5 */

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug)]
struct CrateStacks {
    stacks: Vec<Vec<char>>, // 0-based
}

impl CrateStacks {
    /// Parses [CrateStacks] from the folloing text representation:
    ///
    ///            [G]         [D]     [Q]    
    ///    [P]     [T]         [L] [M] [Z]    
    ///    [Z] [Z] [C]         [Z] [G] [W]    
    ///    [M] [B] [F]         [P] [C] [H] [N]
    ///    [T] [S] [R]     [H] [W] [R] [L] [W]
    ///    [R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
    ///    [C] [N] [H] [R] [N] [H] [D] [J] [Q]
    ///    [N] [D] [M] [G] [Z] [F] [W] [S] [S]
    ///     1   2   3   4   5   6   7   8   9
    fn from_text_repr(mut stacks_text: Vec<&str>) -> CrateStacks {
        let index_line = stacks_text.pop().unwrap();
        stacks_text.reverse();
        let mut stacks_vec = Vec::with_capacity(10);
        for num_match in NUMBER_REGEX.find_iter(index_line) {
            let num = num_match.as_str();
            let num_idx = index_line.find(num).unwrap();
            let mut v = vec![];
            for line in stacks_text.iter() {
                if num_idx < line.len() {
                    let c = line.as_bytes()[num_idx] as char;
                    if c.is_ascii_alphabetic() {
                        v.push(c);
                    }
                }
            }
            stacks_vec.push(v);
        }
        CrateStacks { stacks: stacks_vec }
    }

    fn apply(&mut self, command: &Command) {
        for _ in 0..(command.amount) {
            if let Some(item) = self.stacks.get_mut(command.from - 1).unwrap().pop() {
                let target_stack = self.stacks.get_mut(command.to - 1).unwrap();
                target_stack.push(item);
            } else {
                eprintln!("Unable to execute command: {:?}. Source is empty", command);
            }
        }
    }

    fn peek_top_letters(&self) -> Vec<Option<&char>> {
        self.stacks.iter().map(|vec| vec.last()).collect()
    }
}

#[derive(Debug)]
struct Command {
    from: usize,
    to: usize,
    amount: usize,
}

impl From<&str> for Command {
    /// Convert smth like line below into a [Command]
    ///
    /// move 7 from 6 to 8
    fn from(value_str: &str) -> Self {
        let mut matches = NUMBER_REGEX.find_iter(value_str);
        let amount: usize = matches.next().unwrap().as_str().parse().unwrap();
        let from: usize = matches.next().unwrap().as_str().parse().unwrap();
        let to: usize = matches.next().unwrap().as_str().parse().unwrap();
        Command { from, to, amount }
    }
}

fn process_and_peek_top_letters() -> Result<String, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(utils::find_empirically("day5/part1_input.txt"))?;

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
        stack.apply(&command);
    }

    Ok(stack
        .peek_top_letters()
        .iter()
        .map(|&e| e.unwrap_or(&('-')))
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let top_letters = process_and_peek_top_letters()?;
    println!("Top letters are: {top_letters}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(
            process_and_peek_top_letters().unwrap(),
            String::from("RTGWZTHLD")
        );
    }
}
