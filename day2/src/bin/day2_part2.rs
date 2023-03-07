/*! See https://adventofcode.com/2022/day/2
 * Part 2
 */

use day2::{Choice, Round};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_points: u32 = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|round_str| parse_round(round_str).unwrap())
        .map(|(enemy, desired_outcome)| choose_strategy(enemy, desired_outcome))
        .map(|round| round.total_points())
        .sum();
    println!("Total points: {total_points}");
    Ok(())
}
