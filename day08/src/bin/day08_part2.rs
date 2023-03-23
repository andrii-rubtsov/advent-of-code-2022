/*! See https://adventofcode.com/2022/day/8 */

use day08::Forest;
use std::io::Read;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
struct ScenicForest {
    forest: Forest,
    scenic_score: Vec<Vec<usize>>,
}

impl ScenicForest {
    fn from_forest(forest: Forest) -> ScenicForest {
        let mut scenic_score: Vec<Vec<usize>> = vec![];
        for row in &forest.trees {
            scenic_score.push(vec![0; row.len()]);
        }
        ScenicForest {
            forest,
            scenic_score,
        }
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let height = self.forest.height();
        let width = self.forest.width();
        if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
            return 0;
        }
        let trees = &self.forest.trees;
        let (mut up, mut down, mut left, mut right) = (1, 1, 1, 1);

        while row - up > 0 && trees[row - up][col] < trees[row][col] {
            up += 1;
        }
        while row + down < height - 1 && trees[row + down][col] < trees[row][col] {
            down += 1;
        }
        while col - left > 0 && trees[row][col - left] < trees[row][col] {
            left += 1;
        }
        while col + right < width - 1 && trees[row][col + right] < trees[row][col] {
            right += 1;
        }
        up * down * left * right
    }

    fn calc_scenic_scores(&mut self) {
        for row in 0..(self.forest.height()) {
            for col in 0..(self.forest.width()) {
                self.scenic_score[row][col] = self.scenic_score(row, col);
            }
        }
    }

    fn find_max_scenic_score(&self) -> Option<usize> {
        self.scenic_score
            .iter()
            .flat_map(|row| row.iter().max())
            .max()
            .copied()
    }
}

fn find_max_scenic_score(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let forest = Forest::from_reader(reader)?;
    let mut visible_forest = ScenicForest::from_forest(forest);
    visible_forest.calc_scenic_scores();
    Ok(visible_forest.find_max_scenic_score().unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let max_scenic_score = find_max_scenic_score(asset.data.as_ref())?;
    println!("Max scenic score: {max_scenic_score}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let visible_trees_count = find_max_scenic_score(asset.data.as_ref()).unwrap();
        assert_eq!(visible_trees_count, 444528);
    }

    #[test]
    fn test_test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let visible_trees_count = find_max_scenic_score(asset.data.as_ref()).unwrap();
        assert_eq!(visible_trees_count, 8);
    }
}
