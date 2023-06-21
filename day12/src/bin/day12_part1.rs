/*! See https://adventofcode.com/2022/day/12 */

use std::io::Read;

use day12::{shortest_path, Topology};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn find_shortest_path(reader: impl Read) -> usize {
    let mut topology = Topology::parse(reader);

    let start = topology.get_start();
    shortest_path(&mut topology, start)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset = Asset::get("input.txt").unwrap();
    let reader = asset.data.as_ref();

    let shortest_path: usize = find_shortest_path(reader);

    println!("Shortest path: {shortest_path}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let shortest_path = find_shortest_path(asset.data.as_ref());
        assert_eq!(shortest_path, 31);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let shortest_path = find_shortest_path(asset.data.as_ref());
        assert_eq!(shortest_path, 361);
    }
}
