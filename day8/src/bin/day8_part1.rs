/*! See https://adventofcode.com/2022/day/8 */

use day8::Forest;
use std::io::Read;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
struct VisibleForest {
    forest: Forest,
    visible: Vec<Vec<bool>>,
}

impl VisibleForest {
    fn from_forest(forest: Forest) -> VisibleForest {
        let mut visible: Vec<Vec<bool>> = vec![];
        for row in &forest.trees {
            visible.push(vec![false; row.len()]);
        }
        VisibleForest { forest, visible }
    }

    fn mark_visible(&mut self) {
        let forest = &self.forest;
        let width = forest.width();
        let height = forest.height();

        // mark edges as visible
        for row in 0..height {
            self.visible[row][0] = true;
            self.visible[row][width - 1] = true;
        }
        for col in 0..width {
            self.visible[0][col] = true;
            self.visible[height - 1][col] = true;
        }

        // from up
        for col in 1..width {
            let mut max = forest.trees[0][col];
            for row in 1..height {
                self.visible[row][col] |= forest.trees[row][col] > max;
                max = max.max(forest.trees[row][col]);
            }
        }

        // from bottom
        for col in 1..width {
            let mut max = forest.trees[height - 1][col];
            for row in (1..height).rev() {
                self.visible[row][col] |= forest.trees[row][col] > max;
                max = max.max(forest.trees[row][col]);
            }
        }

        // from left
        for row in 1..height {
            let mut max = forest.trees[row][0];
            for col in 1..width {
                self.visible[row][col] |= forest.trees[row][col] > max;
                max = max.max(forest.trees[row][col]);
            }
        }

        // from right
        for row in 1..height {
            let mut max = forest.trees[row][width - 1];
            for col in (1..width).rev() {
                self.visible[row][col] |= forest.trees[row][col] > max;
                max = max.max(forest.trees[row][col]);
            }
        }
    }

    fn count_visible(&self) -> usize {
        self.visible
            .iter()
            .map(|row| row.iter().map(|&v| v as usize).sum::<usize>())
            .sum()
    }
}

fn count_visible_trees(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let forest = Forest::from_reader(reader)?;
    let mut visible_forest = VisibleForest::from_forest(forest);
    visible_forest.mark_visible();
    Ok(visible_forest.count_visible())
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
