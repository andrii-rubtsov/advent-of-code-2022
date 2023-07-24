#![feature(lazy_cell)]

use std::{sync::LazyLock, collections::{HashSet, HashMap}, io::{Read, BufReader, BufRead}};

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

pub fn parse_nodes(reader: impl Read) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    let mut label_to_node_idx: HashMap<String, usize> = HashMap::new();

    // Initial parsing of labels
    for (idx, maybe_line) in BufReader::new(reader).lines().enumerate() {
        let line = maybe_line.unwrap();
        let node = Node::parse(line);
        label_to_node_idx.insert(node.label.clone(), idx);
        nodes.push(node);
    }

    // Convert and assign nodes labels to indexes for more optimal and convenient access
    for node in nodes.iter_mut() {
        node.tunnels_idx = node
            .tunnels_labels
            .iter()
            .map(|label| *label_to_node_idx.get(label.as_str()).unwrap())
            .collect();
    }

    assert!(
        nodes.len() <= 128,
        "`CacheKey` struct is specifically optimized for less than 128 nodes"
    );
    nodes
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Idle,
    Move(usize),
    Open(usize),
}


pub fn decode_open_valves_indexes(open_valves: u128) -> HashSet<usize> {
    (0..128)
        .filter(|idx| open_valves & (1 << idx) > 0)
        .collect()
}