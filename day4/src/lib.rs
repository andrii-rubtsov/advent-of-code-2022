#[derive(Debug)]
pub struct Range {
    start: u32,
    end: u32,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let splits: Vec<_> = s.split('-').map(|s| s.parse::<u32>().unwrap()).collect();
        if splits.len() != 2 {
            panic!(
                "Expected ranges to be in format `number-number`, but found: {}",
                s
            )
        }
        Range {
            start: *splits.first().unwrap(),
            end: *splits.get(1).unwrap(),
        }
    }
}

impl Range {
    pub fn contains_fully(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        !(self.end < other.start || self.start > other.end)
    }
}
