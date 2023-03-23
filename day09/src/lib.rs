#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn dist(&self, other: &Pos) -> usize {
        i32::max(i32::abs(self.x - other.x), i32::abs(self.y - other.y)) as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        use Direction::*;

        if value.len() != 1 {
            panic!("Value was expected to be a single ASCII char, but was {value}")
        }

        match value.chars().next().unwrap() {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("Unsupported direction"),
        }
    }
}

pub struct Movement {
    pub direction: Direction,
    pub steps: usize,
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split_ascii_whitespace().collect();
        if parts.len() != 2 {
            panic!("Movement was expected to be in form `direction` `steps`, but was {value}")
        }

        let direction: Direction = parts[0].into();
        let steps: usize = parts[1].parse().unwrap();
        Movement { direction, steps }
    }
}
