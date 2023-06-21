use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Read},
};

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }
}

#[derive(Clone)]
pub struct Topology {
    pub heights: Vec<Vec<usize>>,
    pub visited: Vec<Vec<bool>>,
    start: Point,
    end: Point,
}

impl Topology {
    pub fn parse(reader: impl Read) -> Topology {
        let mut heights: Vec<Vec<usize>> = vec![];
        let mut visited: Vec<Vec<bool>> = vec![];

        let mut start = Point::default();
        let mut end = Point::default();

        for (row, input_line) in BufReader::new(reader).lines().enumerate() {
            let line = input_line.unwrap();
            if let Some(col) = line.find('S') {
                start = Point::new(row, col);
            }
            if let Some(col) = line.find('E') {
                end = Point::new(row, col);
            }
            let line_heights: Vec<usize> = line
                .chars()
                .map(|ch| {
                    if ch == 'S' {
                        'a'
                    } else if ch == 'E' {
                        'z'
                    } else {
                        ch
                    }
                })
                .map(|ch| (ch as usize) - ('a' as usize))
                .collect();

            visited.push(vec![false; line_heights.len()]);
            heights.push(line_heights);
        }

        Topology {
            heights,
            visited,
            start,
            end,
        }
    }

    pub fn height_at(&self, p: &Point) -> usize {
        self.heights[p.row][p.col]
    }

    pub fn is_visited(&self, p: &Point) -> bool {
        self.visited[p.row][p.col]
    }

    pub fn get_neighbours(&self, point: &Point) -> Vec<Point> {
        let mut result = vec![];
        if point.col > 0 {
            result.push(Point::new(point.row, point.col - 1));
        }
        if point.col + 1 < self.cols() {
            result.push(Point::new(point.row, point.col + 1));
        }
        if point.row > 0 {
            result.push(Point::new(point.row - 1, point.col));
        }
        if point.row + 1 < self.rows() {
            result.push(Point::new(point.row + 1, point.col));
        }
        result
    }

    pub fn get_start(&self) -> Point {
        self.start.clone()
    }

    pub fn get_end(&self) -> Point {
        self.end.clone()
    }

    pub fn rows(&self) -> usize {
        self.heights.len()
    }

    pub fn cols(&self) -> usize {
        self.heights[0].len()
    }
}

pub fn shortest_path(topology: &mut Topology, start: Point) -> usize {
    let end = topology.get_end();

    let mut path_len = 0;
    let mut next_queue = VecDeque::from([start]);
    let mut queue;

    loop {
        queue = next_queue;
        next_queue = VecDeque::new();

        while let Some(p) = queue.pop_front() {
            if p == end {
                return path_len;
            }
            if topology.is_visited(&p) {
                continue;
            }
            for n in topology.get_neighbours(&p) {
                if !topology.is_visited(&n) && topology.height_at(&n) <= topology.height_at(&p) + 1
                {
                    next_queue.push_back(n);
                }
            }
            topology.visited[p.row][p.col] = true;
        }
        path_len += 1;

        // To avoid endless loop if there is no valid path from start to end.
        // Consider proper error handling
        if next_queue.is_empty() {
            return usize::MAX;
        }
    }
}
