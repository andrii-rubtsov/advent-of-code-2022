/*! See https://adventofcode.com/2022/day/12 */

#![allow(non_upper_case_globals)]

use std::io::Read;

use day12::{shortest_path, Point, Topology};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn find_best_trail_len(reader: impl Read) -> usize {
    let topology = Topology::parse(reader);

    let mut starting_points = vec![];
    for r in 0..topology.rows() {
        for c in 0..topology.cols() {
            let p = Point::new(r, c);
            if topology.height_at(&p) == 0 {
                starting_points.push(p);
            }
        }
    }

    starting_points
        .into_iter()
        .map(|sp| shortest_path(&mut topology.clone(), sp))
        .min()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let shortest_path = find_best_trail_len(asset.data.as_ref());
    println!("Shortest path: {shortest_path}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        assert_eq!(find_best_trail_len(asset.data.as_ref()), 29);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        assert_eq!(find_best_trail_len(asset.data.as_ref()), 354);
    }
}
