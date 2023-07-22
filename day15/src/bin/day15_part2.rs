/*! See https://adventofcode.com/2022/day/15 */

use day15::{manhattan_dist, Pos, COORDS_REGEX};
use rust_embed::RustEmbed;
use std::{
    io::{BufRead, BufReader, Read},
    ops::RangeInclusive,
};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
struct LineFragment {
    start: Pos,
    dist: u32,
    incline: i32, // +1 or -1
}

impl LineFragment {
    fn new(start: Pos, dist: u32, incline: i32) -> Self {
        Self {
            start,
            dist,
            incline,
        }
    }

    fn intersection(&self, other: &LineFragment) -> Option<Pos> {
        // It *should* be enough to only consider crossing of lines with opposite-sign inclines
        if self.incline + other.incline != 0 {
            return None;
        }
        let (b1, b2) = if self.incline == -1 {
            (self.start.y - self.start.x, other.start.y + other.start.x)
        } else {
            (self.start.y + self.start.x, other.start.y - other.start.x)
        };
        let candidate = Pos::new((b2 - b1) / 2, (b2 + b1) / 2);
        if self.start.dist_to(&candidate) <= self.dist + 1
            && other.start.dist_to(&candidate) <= other.dist + 1
        {
            Some(candidate)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    radius: u32,
}

impl Sensor {
    fn new(pos: Pos, radius: u32) -> Sensor {
        Sensor { pos, radius }
    }

    fn border_line_fragments(&self) -> Vec<LineFragment> {
        let radius = self.radius as i32 + 1;
        let left = Pos::new(self.pos.x - radius, self.pos.y);
        let top = Pos::new(self.pos.x, self.pos.y + radius);
        let bottom = Pos::new(self.pos.x, self.pos.y - radius);

        vec![
            LineFragment::new(left.clone(), 2 * radius as u32, 1),
            LineFragment::new(left.clone(), 2 * radius as u32, -1),
            LineFragment::new(top, 2 * radius as u32, 1),
            LineFragment::new(bottom, 2 * radius as u32, -1),
        ]
    }
}

struct Map {
    sensors: Vec<Sensor>,
    soloution_x_range: RangeInclusive<i32>,
    soloution_y_range: RangeInclusive<i32>,
}

impl Map {
    pub fn parse(
        reader: impl Read,
        soloution_x_range: RangeInclusive<i32>,
        soloution_y_range: RangeInclusive<i32>,
    ) -> Map {
        let mut sensors = vec![];
        for maybe_line in BufReader::new(reader).lines() {
            let line = maybe_line.unwrap();
            let captures = COORDS_REGEX.captures_iter(&line).next().unwrap();

            let sensor_x: i32 = captures.name("sensor_x").unwrap().as_str().parse().unwrap();
            let sensor_y: i32 = captures.name("sensor_y").unwrap().as_str().parse().unwrap();
            let beacon_x: i32 = captures.name("beacon_x").unwrap().as_str().parse().unwrap();
            let beacon_y: i32 = captures.name("beacon_y").unwrap().as_str().parse().unwrap();

            let sensor_pos = Pos::new(sensor_x, sensor_y);
            let beacon_pos = Pos::new(beacon_x, beacon_y);
            let radius = manhattan_dist(&sensor_pos, &beacon_pos);
            sensors.push(Sensor::new(sensor_pos, radius));
        }
        Map {
            sensors,
            soloution_x_range,
            soloution_y_range,
        }
    }

    fn is_solution(&self, pos: &Pos) -> bool {
        for sensor in self.sensors.iter() {
            if manhattan_dist(&sensor.pos, pos) <= sensor.radius {
                return false;
            }
        }
        self.soloution_x_range.contains(&pos.x) && self.soloution_y_range.contains(&pos.y)
    }
}

fn find_distress_signal(map: &mut Map) -> Pos {
    let line_fragments = map
        .sensors
        .iter()
        .flat_map(|sensor| sensor.border_line_fragments())
        .collect::<Vec<_>>();

    for i in 0..line_fragments.len() {
        for j in i..line_fragments.len() {
            let fragment1 = &line_fragments[i];
            let fragment2 = &line_fragments[j];

            // Eevry intersection of border line framgents is a potential distress signal candidate
            // By checking only "diamond" borders and their interscections the algorith significantly narrows
            // down the scope of potential candiadtes.
            if let Some(candidate) = fragment1.intersection(fragment2) {
                //println!("Intersection candiate: {candidate}");
                if map.is_solution(&candidate) {
                    return candidate;
                }
            }
        }
    }
    unreachable!("Was not able to find a solution")
}

fn find_distress_and_frequency(
    reader: impl Read,
    solution_x_range: RangeInclusive<i32>,
    solution_y_range: RangeInclusive<i32>,
) -> (Pos, u64) {
    let mut map = Map::parse(reader, solution_x_range, solution_y_range);

    let distress_signal: Pos = find_distress_signal(&mut map);

    let frequency = distress_signal.x as u64 * 4_000_000_u64 + distress_signal.y as u64;
    (distress_signal, frequency)
}

fn main() {
    let asset = Asset::get("input.txt").unwrap();
    let distress_signal_and_frequency =
        find_distress_and_frequency(asset.data.as_ref(), 0..=4_000_000, 0..=4_000_000);
    println!("Distress signal: {}", distress_signal_and_frequency.0);
    println!(
        "Distress signal frequency: {}",
        distress_signal_and_frequency.1
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_solution() {
        let asset = Asset::get("test_input.txt").unwrap();
        let map = Map::parse(asset.data.as_ref(), 0..=20, 0..=20);
        assert!(map.is_solution(&Pos::new(14, 11)));
    }

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let distress_signal_and_frequency =
            find_distress_and_frequency(asset.data.as_ref(), 0..=20, 0..=20);
        assert_eq!(distress_signal_and_frequency.0, Pos::new(14, 11));
        assert_eq!(distress_signal_and_frequency.1, 56000011);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let distress_signal_and_frequency =
            find_distress_and_frequency(asset.data.as_ref(), 0..=4_000_000, 0..=4_000_000);
        assert_eq!(distress_signal_and_frequency.0, Pos::new(2572895, 2906626));
        assert_eq!(distress_signal_and_frequency.1, 10291582906626);
    }
}
