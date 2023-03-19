/*! See https://adventofcode.com/2022/day/9 */

use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader, Read},
};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn dist(&self, other: &Pos) -> usize {
        i32::max(i32::abs(self.x - other.x), i32::abs(self.y - other.y)) as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        use Direction::*;

        if value.len() != 1 {
            panic!("Value was expected to be a single ASCII char, but was {value}")
        }

        match value.chars().next().unwrap() {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("Unsupported direction"),
        }
    }
}

pub struct Movement {
    direction: Direction,
    steps: usize,
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split_ascii_whitespace().collect();
        if parts.len() != 2 {
            panic!("Movement was expected to be in form `direction` `steps`, but was {value}")
        }

        let direction: Direction = parts[0].into();
        let steps: usize = parts[1].parse().unwrap();
        Movement { direction, steps }
    }
}

#[derive(Debug)]
pub struct BridgeMotions {
    tail_positions: HashSet<Pos>,
    head: Pos,
    tail: Pos,
}

impl Default for BridgeMotions {
    fn default() -> Self {
        let initial_pos = Pos::default();
        let mut tail_positions = HashSet::new();
        tail_positions.insert(initial_pos);
        Self {
            tail_positions,
            head: initial_pos,
            tail: initial_pos,
        }
    }
}

impl BridgeMotions {
    pub fn process_head_movement(&mut self, head_movement: &Movement) {
        use Direction::*;

        match head_movement.direction {
            Up => self.head.y += head_movement.steps as i32,
            Down => self.head.y -= head_movement.steps as i32,
            Right => self.head.x += head_movement.steps as i32,
            Left => self.head.x -= head_movement.steps as i32,
        }
        while self.head.dist(&self.tail) > 1 {
            let new_tail_pos: Pos = match head_movement.direction {
                Up => Pos::new(self.head.x, self.tail.y + 1),
                Down => Pos::new(self.head.x, self.tail.y - 1),
                Right => Pos::new(self.tail.x + 1, self.head.y),
                Left => Pos::new(self.tail.x - 1, self.head.y),
            };
            self.tail = new_tail_pos;
            self.tail_positions.insert(new_tail_pos);
        }
    }
}

fn total_unique_tail_locations(reader: impl Read) -> Result<usize, io::Error> {
    let mut bridge_motions = BridgeMotions::default();
    for line in BufReader::new(reader).lines() {
        let movement: Movement = line?.as_str().into();
        bridge_motions.process_head_movement(&movement);
    }
    Ok(bridge_motions.tail_positions.len())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let total_unique_tail_locations = total_unique_tail_locations(asset.data.as_ref())?;
    println!("Total unique tail locations: {total_unique_tail_locations}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let total_unique_tail_locations = total_unique_tail_locations(asset.data.as_ref()).unwrap();
        assert_eq!(total_unique_tail_locations, 13);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let total_unique_tail_locations = total_unique_tail_locations(asset.data.as_ref()).unwrap();
        assert_eq!(total_unique_tail_locations, 6090);
    }
}
