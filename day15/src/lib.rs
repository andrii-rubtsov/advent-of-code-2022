#![feature(lazy_cell)]

use std::{fmt::Display, sync::LazyLock};

use regex::Regex;

// Sensor at x=2557568, y=3759110: closest beacon is at x=2594124, y=3746832
pub static COORDS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r".*x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+).*x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap()
});

pub fn manhattan_dist(a: &Pos, b: &Pos) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn dist_to(&self, other: &Pos) -> u32 {
        manhattan_dist(self, other)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.x, self.y)
    }
}
