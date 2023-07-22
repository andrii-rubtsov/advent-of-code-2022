/*! See https://adventofcode.com/2022/day/15 */

use day15::{manhattan_dist, Pos, COORDS_REGEX};
use rust_embed::RustEmbed;
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

pub struct Map {
    pub sensors_to_beacons: HashMap<Pos, Pos>,
}

impl Map {
    pub fn parse(reader: impl Read) -> Map {
        let mut sensors_to_beacons = HashMap::new();
        for maybe_line in BufReader::new(reader).lines() {
            let line = maybe_line.unwrap();
            let captures = COORDS_REGEX.captures_iter(&line).next().unwrap();
            let sensor_x: i32 = captures.name("sensor_x").unwrap().as_str().parse().unwrap();
            let sensor_y: i32 = captures.name("sensor_y").unwrap().as_str().parse().unwrap();
            let beacon_x: i32 = captures.name("beacon_x").unwrap().as_str().parse().unwrap();
            let beacon_y: i32 = captures.name("beacon_y").unwrap().as_str().parse().unwrap();

            sensors_to_beacons.insert(Pos::new(sensor_x, sensor_y), Pos::new(beacon_x, beacon_y));
        }
        Map { sensors_to_beacons }
    }
}

fn ruled_out_beacon_pos(reader: impl Read, target_y: i32) -> usize {
    let map = Map::parse(reader);
    let known_beacons_on_target = map
        .sensors_to_beacons
        .values()
        .filter_map(|pos| if pos.y == target_y { Some(pos.x) } else { None })
        .collect::<HashSet<_>>();

    let mut ruled_out_x = HashSet::new();
    for (sensor, beacon) in map.sensors_to_beacons.iter() {
        let sensor_to_beacon_dist = manhattan_dist(sensor, beacon);
        let sensor_to_target_dist = sensor.y.abs_diff(target_y);
        if sensor_to_beacon_dist < sensor_to_target_dist {
            continue;
        }
        let delta = (sensor_to_beacon_dist - sensor_to_target_dist) as i32;
        for x in (sensor.x - delta)..=(sensor.x + delta) {
            if known_beacons_on_target.get(&x).is_none() {
                ruled_out_x.insert(x);
            }
        }
    }
    ruled_out_x.len()
}

fn main() {
    let asset = Asset::get("input.txt").unwrap();
    let ruled_out_beacon_pos = ruled_out_beacon_pos(asset.data.as_ref(), 2000000);
    println!("Total positions that cannot contain a beacon: {ruled_out_beacon_pos}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let sand_units_came_to_rest = ruled_out_beacon_pos(asset.data.as_ref(), 10);
        assert_eq!(sand_units_came_to_rest, 26);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let sand_units_came_to_rest = ruled_out_beacon_pos(asset.data.as_ref(), 2000000);
        assert_eq!(sand_units_came_to_rest, 5403290);
    }
}
