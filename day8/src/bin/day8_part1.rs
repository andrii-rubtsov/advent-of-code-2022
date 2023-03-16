/*! See https://adventofcode.com/2022/day/8 */

use std::io::{self, BufRead, BufReader, Read};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
pub struct Asset;

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<u8>>,
    visible: Vec<Vec<bool>>,
}

impl Forest {
    fn height(&self) -> usize {
        self.trees.len()
    }

    fn width(&self) -> usize {
        self.trees[0].len()
    }

    fn from_reader(reader: impl Read) -> Result<Forest, io::Error> {
        let mut trees: Vec<Vec<u8>> = vec![];
        let mut visible: Vec<Vec<bool>> = vec![];

        for line in BufReader::new(reader).lines() {
            trees.push(line?.chars().map(|c| c as u8).collect());
            visible.push(vec![false; trees.last().unwrap().len()]);
        }

        Ok(Forest { trees, visible })
    }

    fn count_visible(&self) -> usize {
        self.visible
            .iter()
            .map(|row| row.iter().map(|&v| v as usize).sum::<usize>())
            .sum()
    }
}

fn mark_visible(forest: &mut Forest) {
    let width = forest.width();
    let height = forest.height();

    // mark edges as visible
    for row in 0..height {
        forest.visible[row][0] = true;
        forest.visible[row][width - 1] = true;
    }
    for col in 0..width {
        forest.visible[0][col] = true;
        forest.visible[height - 1][col] = true;
    }

    // from up
    for col in 1..width {
        let mut max = forest.trees[0][col];
        for row in 1..height {
            forest.visible[row][col] |= forest.trees[row][col] > max;
            max = max.max(forest.trees[row][col]);
        }
    }

    // from bottom
    for col in 1..width {
        let mut max = forest.trees[height - 1][col];
        for row in (1..height).rev() {
            forest.visible[row][col] |= forest.trees[row][col] > max;
            max = max.max(forest.trees[row][col]);
        }
    }

    // from left
    for row in 1..height {
        let mut max = forest.trees[row][0];
        for col in 1..width {
            forest.visible[row][col] |= forest.trees[row][col] > max;
            max = max.max(forest.trees[row][col]);
        }
    }

    // from right
    for row in 1..height {
        let mut max = forest.trees[row][width - 1];
        for col in (1..width).rev() {
            forest.visible[row][col] |= forest.trees[row][col] > max;
            max = max.max(forest.trees[row][col]);
        }
    }
}

fn count_visible_trees(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let mut forest = Forest::from_reader(reader)?;
    mark_visible(&mut forest);
    Ok(forest.count_visible())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let visible_trees_count = count_visible_trees(asset.data.as_ref())?;
    println!("Visible trees count: {visible_trees_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let visible_trees_count = count_visible_trees(asset.data.as_ref()).unwrap();
        assert_eq!(visible_trees_count, 1805);
    }

    #[test]
    fn test_test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let visible_trees_count = count_visible_trees(asset.data.as_ref()).unwrap();
        assert_eq!(visible_trees_count, 21);
    }
}
