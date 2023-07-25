#![feature(lazy_cell)]

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    sync::LazyLock,
};

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

pub struct PathFinder<'a> {
    pub nodes: &'a [Node],
    cache: HashMap<(usize, usize), Vec<usize>>,
}

impl<'a> PathFinder<'a> {
    pub fn new(nodes: &'a [Node]) -> Self {
        PathFinder {
            nodes,
            cache: HashMap::new(),
        }
    }

    pub fn new_with_cache(all_nodes: &'a [Node], nodes: &[usize]) -> Self {
        let mut pf = PathFinder::new(all_nodes);
        pf.initialize_cache(nodes);
        pf
    }

    pub fn shortest_path_len(&mut self, from: usize, to: usize) -> usize {
        let key = (from, to);
        assert!(from != to);

        if self.cache.contains_key(&key) {
            return self.cache.get(&key).unwrap().len();
        }
        let path = self.breadth_first_search(from, to);
        match path {
            None => unreachable!("Isoalted island of node are not expected!"),
            Some(mut path) => {
                let ans = path.len();
                self.cache.insert(key, path.clone());
                path.reverse();
                self.cache.insert((to, from), path);
                ans
            }
        }
    }

    pub fn shortest_path_cached(&self, from: usize, to: usize) -> &[usize] {
        self.cache.get(&(from, to)).unwrap()
    }

    pub fn shortest_path(&mut self, from: usize, to: usize) -> &[usize] {
        let key = (from, to);
        assert!(from != to);

        if self.cache.contains_key(&key) {
            return self.cache.get(&key).unwrap();
        }
        let result = self.breadth_first_search(from, to);
        match result {
            None => unreachable!("Isolated island of node are not expected!"),
            Some(mut lineage) => {
                self.cache.insert(key, lineage.clone());
                let rev_key = (to, from);
                lineage.reverse();
                self.cache.insert(rev_key, lineage);
                self.cache.get(&key).unwrap()
            }
        }
    }

    fn breadth_first_search(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        let mut from_queue: Vec<usize> = vec![from];
        let mut to_queue: Vec<usize> = vec![to];

        let mut from_queue_next: Vec<usize> = vec![];
        let mut to_queue_next: Vec<usize> = vec![];

        let mut from_lineage = HashMap::new();
        from_lineage.insert(from, vec![from]);

        let mut to_lineage = HashMap::new();
        to_lineage.insert(to, vec![to]);

        while !from_lineage.is_empty() || !to_lineage.is_empty() {
            for from_elem in from_queue {
                for &neighbour in self.nodes[from_elem].tunnels_idx.iter() {
                    if from_lineage.contains_key(&neighbour) {
                        // already seen, skip
                        continue;
                    }
                    if let Some(neighbour_lineage) = to_lineage.get(&neighbour) {
                        // bingo "from" wave meats "to" wave
                        let mut neighbour_lineage = neighbour_lineage.clone();
                        // reverse neighbour's lineage because it is part of "to" wave
                        neighbour_lineage.reverse();

                        let mut result = from_lineage.get(&from_elem).unwrap().clone();
                        result.append(&mut neighbour_lineage);

                        return Some(result);
                    }
                    // else, just create new neighbour's lineage and continue
                    let mut neighbour_lineage = from_lineage.get(&from_elem).unwrap().clone();
                    neighbour_lineage.push(neighbour);
                    from_lineage.insert(neighbour, neighbour_lineage);
                    from_queue_next.push(neighbour);
                }
            }
            for to_elem in to_queue {
                for &neighbour in self.nodes[to_elem].tunnels_idx.iter() {
                    if to_lineage.contains_key(&neighbour) {
                        // already seen, skip
                        continue;
                    }
                    if let Some(neighbour_lineage) = from_lineage.get(&neighbour) {
                        // bingo "to" wave meats "from" wave
                        // reverse own lineage because it is part of "to" wave
                        let mut result = neighbour_lineage.clone();
                        let mut self_lineage = to_lineage.get(&to_elem).unwrap().clone();
                        self_lineage.reverse();
                        result.append(&mut self_lineage);

                        return Some(result);
                    }
                    // else, just create new neighbour's lineage and continue
                    let mut neighbour_lineage = to_lineage.get(&to_elem).unwrap().clone();
                    neighbour_lineage.push(neighbour);
                    to_lineage.insert(neighbour, neighbour_lineage);
                    to_queue_next.push(neighbour);
                }
            }
            from_queue = from_queue_next;
            to_queue = to_queue_next;

            from_queue_next = vec![];
            to_queue_next = vec![];
        }

        // ok, if we reached this line it means that there are isolated isaland of nodes,
        // which should not happen for the input data that we have seen but it is possible in theory
        None
    }

    pub fn initialize_cache(&mut self, relevant_nodes: &[usize]) {
        for from in relevant_nodes.iter() {
            for to in relevant_nodes.iter() {
                if from == to {
                    continue;
                }
                self.shortest_path(*from, *to);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Idle,
    Move(usize),
    Open(usize),
}

pub fn generate_possible_routes(
    path_finder: &PathFinder,
    initial: usize,
    nodes_to_open: &[usize],
    time_remaining: usize,
) -> Vec<Vec<Action>> {
    let mut ans: Vec<Vec<Action>> = Vec::new();

    for (idx, &node) in nodes_to_open.iter().enumerate() {
        let path = path_finder.shortest_path_cached(initial, node);
        if path.len() >= time_remaining {
            continue;
        }
        let mut actions = Vec::from_iter(path.iter().skip(1).map(|&n| Action::Move(n)));
        actions.push(Action::Open(node));

        let mut new_nodes = Vec::with_capacity(nodes_to_open.len() - 1);
        if idx > 0 {
            new_nodes.append(&mut nodes_to_open[..idx].to_vec());
        }
        if idx < nodes_to_open.len() - 1 {
            new_nodes.append(&mut nodes_to_open[idx + 1..].to_vec());
        }

        let next_actions =
            generate_possible_routes(path_finder, node, &new_nodes, time_remaining - path.len());

        // always add a route that preemptivbely stops and doest no go further
        // this is to to avoid scenarios when the main character always attempts to
        // cover more oepn valves then optimally needed.

        ans.push(actions.clone());

        for mut next_actions_chain in next_actions {
            let mut next = actions.clone();
            next.append(&mut next_actions_chain);
            ans.push(next);
        }
    }
    ans
}

pub fn calc_pressure_released(
    actions: &Vec<Action>,
    nodes: &[Node],
    time_budget: usize,
) -> (usize, Vec<Action>) {
    let total_released = actions
        .iter()
        .enumerate()
        .filter_map(|(time, a)| match a {
            Action::Open(node_id) => Some(nodes[*node_id].rate * (time_budget - time - 1)),
            _ => None,
        })
        .sum();

    let mut lineage = Vec::from_iter(actions.iter().copied());
    lineage.append(&mut vec![Action::Idle; time_budget - actions.len()]);

    (total_released, lineage)
}

pub struct OptimalResult<T> {
    pub total_pressure_released: usize,
    pub actions_lineage: Vec<T>,
}

impl<T> OptimalResult<T> {
    pub fn new(total_pressure_released: usize, actions_lineage: Vec<T>) -> Self {
        OptimalResult {
            total_pressure_released,
            actions_lineage,
        }
    }
}

#[allow(non_upper_case_globals)]
#[cfg(test)]
mod tests {
    use rust_embed::RustEmbed;

    use super::*;

    #[derive(RustEmbed)]
    #[folder = "."]
    struct Asset;

    #[test]
    fn path_finder_shortest_path() {
        let asset = Asset::get("test_input.txt").unwrap();
        let nodes = parse_nodes(asset.data.as_ref());
        let mut path_finder = PathFinder::new(&nodes);

        assert_eq!(path_finder.shortest_path(0, 5), vec![0, 3, 4, 5]);
        assert_eq!(path_finder.shortest_path(5, 0), vec![5, 4, 3, 0]);
        assert_eq!(path_finder.shortest_path_len(0, 5), 4);
        assert_eq!(path_finder.shortest_path_len(5, 0), 4);

        assert_eq!(path_finder.shortest_path(6, 4), vec![6, 5, 4]);
        assert_eq!(path_finder.shortest_path(4, 6), vec![4, 5, 6]);
        assert_eq!(path_finder.shortest_path_len(6, 4), 3);
        assert_eq!(path_finder.shortest_path_len(4, 6), 3);

        assert_eq!(path_finder.shortest_path(0, 9), vec![0, 8, 9]);
        assert_eq!(path_finder.shortest_path(9, 0), vec![9, 8, 0]);
        assert_eq!(path_finder.shortest_path_len(0, 9), 3);
        assert_eq!(path_finder.shortest_path_len(9, 0), 3);
    }
}
