use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    Sand,
    Rock,
}

pub struct Cave {
    pub cells: HashMap<Pos, Cell>,
    pub max_height: usize,
}

impl Cave {
    /// Parses files of strings like:
    /// 498,4 -> 498,6 -> 496,6
    pub fn parse(reader: impl Read) -> Cave {
        let mut cells = HashMap::new();
        for line in BufReader::new(reader).lines() {
            let line = line.unwrap();
            let chain: Vec<Pos> = line
                .split(" -> ")
                .map(|pair_str| {
                    let pair: Vec<&str> = pair_str.split(',').collect();
                    let x: usize = str::parse(pair[0]).unwrap();
                    let y: usize = str::parse(pair[1]).unwrap();
                    Pos { x, y }
                })
                .collect();

            let mut start = 0;
            let mut end = start + 1;
            while end < chain.len() {
                let a = &chain[start];
                let b = &chain[end];
                if a.x == b.x {
                    // vertical line
                    for y in usize::min(a.y, b.y)..=usize::max(a.y, b.y) {
                        cells.insert(Pos::new(a.x, y), Cell::Rock);
                    }
                } else {
                    // horizontal line
                    for x in usize::min(a.x, b.x)..=usize::max(a.x, b.x) {
                        cells.insert(Pos::new(x, a.y), Cell::Rock);
                    }
                }
                start += 1;
                end += 1;
            }
        }
        let max_height = cells.keys().map(|pos| pos.y).max().unwrap();
        Cave { cells, max_height }
    }

    pub fn get(&self, pos: &Pos) -> &Cell {
        if let Some(cell) = self.cells.get(pos) {
            return cell;
        }
        &Cell::Empty
    }

    pub fn put_sand(&mut self, pos: &Pos) {
        self.cells.insert(pos.clone(), Cell::Sand);
    }

    pub fn count_sand_cells(&self) -> usize {
        self.cells
            .values()
            .filter(|cell| **cell == Cell::Sand)
            .count()
    }
}
