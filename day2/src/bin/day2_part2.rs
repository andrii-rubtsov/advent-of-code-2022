/*! See https://adventofcode.com/2022/day/2
 * Part 2
 */

use day2::{Choice, Round};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

enum DesiredOutcome {
    Lose,
    Draw,
    Win,
}

impl DesiredOutcome {
    fn decode(c: char) -> DesiredOutcome {
        match c {
            'X' => DesiredOutcome::Lose,
            'Y' => DesiredOutcome::Draw,
            'Z' => DesiredOutcome::Win,
            _ => panic!("unexpected desired behavior"),
        }
    }
}

#[derive(Debug)]
struct RoundParseError {
    #[allow(dead_code)]
    pub reason: String,
}

fn parse_round(s: &str) -> Result<(Choice, DesiredOutcome), RoundParseError> {
    let (&enemy_s, &desired_outcome_s) = (
        &s[..1].chars().next().ok_or(RoundParseError {
            reason: "error reading the 1st char in a game round".into(),
        })?,
        &s[2..].chars().next().ok_or(RoundParseError {
            reason: "error reading the 2nd char in a game round".into(),
        })?,
    );
    Ok((
        Choice::decode_enemy(enemy_s),
        DesiredOutcome::decode(desired_outcome_s),
    ))
}

fn choose_strategy(enemy: Choice, desired_outcome: DesiredOutcome) -> Round {
    let own: Choice = match desired_outcome {
        DesiredOutcome::Lose => enemy.wins(),
        DesiredOutcome::Draw => enemy.clone(),
        DesiredOutcome::Win => enemy.looses_to(),
    };
    Round::new(enemy, own)
}

fn total_strategic_points() -> Result<u32, Box<dyn std::error::Error>> {
    let input_resource = Asset::get("input.txt").unwrap();
    let input = std::str::from_utf8(input_resource.data.as_ref())?;
    Ok(input
        .lines()
        .map(|round_str| parse_round(round_str).unwrap())
        .map(|(enemy, desired_outcome)| choose_strategy(enemy, desired_outcome))
        .map(|round| round.total_points())
        .sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_points = total_strategic_points()?;
    println!("Total points: {total_points}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        assert_eq!(total_strategic_points().unwrap(), 14204);
    }
}
