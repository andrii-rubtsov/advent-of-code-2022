/*! See https://adventofcode.com/2022/day/2 */

use std::io::{BufRead, BufReader, Read};

use day02::{Choice, Round};

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
        Choice::decode(enemy_s),
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

fn total_strategic_points(reader: impl Read) -> usize {
    BufReader::new(reader)
        .lines()
        .map(|round_str| parse_round(&round_str.unwrap()).unwrap())
        .map(|(enemy, desired_outcome)| choose_strategy(enemy, desired_outcome))
        .map(|round| round.total_points())
        .sum()
}

fn main() {
    let asset = Asset::get("input.txt").unwrap();
    let total_points = total_strategic_points(asset.data.as_ref());
    println!("Total points: {total_points}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(total_strategic_points(asset.data.as_ref()), 12);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(total_strategic_points(asset.data.as_ref()), 14204);
    }
}
