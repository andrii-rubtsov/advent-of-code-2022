use std::{char::ParseCharError, cmp::Ordering, collections::HashMap, str::FromStr};

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

lazy_static! {
    static ref CHOICE_WINS: HashMap<Choice, Choice> = {
        let mut m = HashMap::with_capacity(3);
        m.insert(Choice::Rock, Choice::Scissors);
        m.insert(Choice::Scissors, Choice::Paper);
        m.insert(Choice::Paper, Choice::Rock);
        m
    };
    static ref CHOICE_LOOSES: HashMap<Choice, Choice> = {
        let mut m = HashMap::with_capacity(3);
        for (key, value) in CHOICE_WINS.iter() {
            m.insert(value.clone(), key.clone());
        }
        m
    };
}

impl Choice {
    pub fn wins(&self) -> Choice {
        CHOICE_WINS.get(self).unwrap().clone()
    }

    pub fn looses_to(&self) -> Choice {
        CHOICE_LOOSES.get(self).unwrap().clone()
    }

    pub fn points(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub fn decode(c: char) -> Choice {
        match c {
            'X'|'A' => Choice::Rock,
            'Y'|'B' => Choice::Paper,
            'Z'|'C' => Choice::Scissors,
            _ => unreachable!(),
        }
    }

}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        let what_self_wins = CHOICE_WINS.get(self).unwrap();
        if what_self_wins == other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

#[derive(Debug)]
pub struct Round {
    pub enemy: Choice,
    pub own: Choice,
}

impl Round {
    pub fn new(enemy: Choice, own: Choice) -> Self {
        Round { enemy, own }
    }

    pub fn total_points(&self) -> usize {
        // 1) unconditionally get points for the chozen strategy
        let mut total = self.own.points();

        // 2) Get 0, 3, or 6 points depending on the round outcome
        if self.own > self.enemy {
            total += 6;
        } else if self.enemy == self.own {
            total += 3;
        }

        total
    }
}

#[derive(Debug)]
pub struct ParseRoundError(String);
impl From<ParseCharError> for ParseRoundError {
    fn from(value: ParseCharError) -> Self {
        ParseRoundError(format!("Error parsing round from '{}'", value))
    }
}

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        let (enemy_s, own_s) = (char::from_str(&s[..1])?, char::from_str(&s[2..])?);
        Ok(Round::new(
            Choice::decode(enemy_s),
            Choice::decode(own_s),
        ))
    }
}
