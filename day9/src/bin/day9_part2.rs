/*! See https://adventofcode.com/2022/day/9 */

use day9::*;
use rust_embed::RustEmbed;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader, Read},
};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
struct MultiknotRopeMotions {
    rope: Vec<Pos>,
    tail_positions: HashSet<Pos>,
}

impl MultiknotRopeMotions {
    fn new(rope_size: usize) -> Self {
        let initial_pos = Pos::default();
        let mut tail_positions = HashSet::new();
        tail_positions.insert(initial_pos);

        let rope = vec![initial_pos; rope_size];
        Self {
            rope,
            tail_positions,
        }
    }

    pub fn process_head_movement(&mut self, head_movement: &Movement) {
        use Direction::*;
        let rope = &mut self.rope;

        'head_steps: for _ in 0..head_movement.steps {
            match head_movement.direction {
                Up => rope[0].y += 1,
                Down => rope[0].y -= 1,
                Right => rope[0].x += 1,
                Left => rope[0].x -= 1,
            }
            for idx in 1..(rope.len()) {
                let parent = rope[idx - 1];
                let child = &mut rope[idx];
                if parent.dist(child) < 2 {
                    continue 'head_steps;
                }
                if parent.x != child.x {
                    child.x += (parent.x - child.x) / i32::abs(parent.x - child.x);
                }
                if parent.y != child.y {
                    child.y += (parent.y - child.y) / i32::abs(parent.y - child.y);
                }
            }
            self.tail_positions.insert(*rope.last().unwrap());
        }
    }
}

fn total_multiknot_rope_tail_locations(reader: impl Read) -> Result<usize, io::Error> {
    let mut rope_motions = MultiknotRopeMotions::new(10);
    for line in BufReader::new(reader).lines() {
        let movement: Movement = line?.as_str().into();
        rope_motions.process_head_movement(&movement);
    }
    Ok(rope_motions.tail_positions.len())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let total_unique_tail_locations = total_multiknot_rope_tail_locations(asset.data.as_ref())?;
    println!("Total unique tail locations: {total_unique_tail_locations}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let actual = total_multiknot_rope_tail_locations(asset.data.as_ref()).unwrap();
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_input_2() {
        let asset = Asset::get("test_input_2.txt").unwrap();
        let actual = total_multiknot_rope_tail_locations(asset.data.as_ref()).unwrap();
        assert_eq!(actual, 36);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let actual = total_multiknot_rope_tail_locations(asset.data.as_ref()).unwrap();
        assert_eq!(actual, 2566);
    }
}
