/*! See https://adventofcode.com/2022/day/9 */

use day09::*;
use rust_embed::RustEmbed;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader, Read},
};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
struct SimpleRopeMotions {
    tail_positions: HashSet<Pos>,
    head: Pos,
    tail: Pos,
}

impl Default for SimpleRopeMotions {
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

impl SimpleRopeMotions {
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
    let mut rope_motions = SimpleRopeMotions::default();
    for line in BufReader::new(reader).lines() {
        let movement: Movement = line?.as_str().into();
        rope_motions.process_head_movement(&movement);
    }
    Ok(rope_motions.tail_positions.len())
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
