#![feature(lazy_cell)]

use std::sync::LazyLock;

use regex::Regex;

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
pub static INPUT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Valve (?P<label>\w+) has flow rate=(?P<rate>\d+); tunnel(s)? lead(s)? to valve(s)? (?P<valves>.*)").unwrap()
});

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Node {
    pub label: String,
    pub rate: usize,
    pub tunnels_labels: Vec<String>,
    pub tunnels_idx: Vec<usize>,
}

impl Node {
    pub fn new(label: String, rate: usize, tunnels_labels: Vec<String>) -> Self {
        Node {
            label,
            rate,
            tunnels_labels,
            tunnels_idx: vec![],
        }
    }

    pub fn parse(input: String) -> Self {
        let captures = INPUT_REGEX.captures_iter(input.as_str()).next().unwrap();
        let label = captures.name("label").unwrap().as_str().to_owned();
        let rate: usize = captures.name("rate").unwrap().as_str().parse().unwrap();
        let tunnels_labels: Vec<String> = captures
            .name("valves")
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        Node::new(label, rate, tunnels_labels)
    }
}
