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

pub type ValveId = usize;

pub type EncodedRouteValvesSet = usize;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Valve {
    pub valve_id: ValveId,
    pub label: String,
    pub rate: usize,
    pub link_valve_labels: Vec<String>,
    pub link_valves_ids: Vec<ValveId>,
}

impl Valve {
    pub fn new(
        valve_id: ValveId,
        label: String,
        rate: usize,
        link_valve_labels: Vec<String>,
    ) -> Self {
        Valve {
            valve_id,
            label,
            rate,
            link_valve_labels,
            link_valves_ids: vec![],
        }
    }

    pub fn parse(input: String, valve_id: ValveId) -> Self {
        let captures = INPUT_REGEX.captures_iter(input.as_str()).next().unwrap();
        let label = captures.name("label").unwrap().as_str().to_owned();
        let rate: usize = captures.name("rate").unwrap().as_str().parse().unwrap();
        let link_valve_labels: Vec<String> = captures
            .name("valves")
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        Valve::new(valve_id, label, rate, link_valve_labels)
    }
}

pub fn parse_valves_network(reader: impl Read) -> Vec<Valve> {
    let mut valves: Vec<Valve> = vec![];
    let mut label_to_valve_ids: HashMap<String, ValveId> = HashMap::new();

    // Initial parsing of labels
    for (valve_id, maybe_line) in BufReader::new(reader).lines().enumerate() {
        let line = maybe_line.unwrap();
        let valve = Valve::parse(line, valve_id);
        label_to_valve_ids.insert(valve.label.clone(), valve_id);
        valves.push(valve);
    }

    // Convert valve labels to valve ids for more optimal and convenient access
    for valve in valves.iter_mut() {
        valve.link_valves_ids = valve
            .link_valve_labels
            .iter()
            .map(|label| *label_to_valve_ids.get(label.as_str()).unwrap())
            .collect();
    }

    assert!(
        valves.len() < 128,
        "Some logic in this crate assumes the total number of valves to be less than 128"
    );
    valves
}

struct PathFinder<'a> {
    pub valve: &'a [Valve],
    cache: HashMap<(ValveId, ValveId), Vec<ValveId>>,
}

impl<'a> PathFinder<'a> {
    pub fn new(valves: &'a [Valve]) -> Self {
        PathFinder {
            valve: valves,
            cache: HashMap::new(),
        }
    }

    pub fn new_with_cache(valves_network: &'a [Valve], valves_to_precache: &[ValveId]) -> Self {
        let mut pf = PathFinder::new(valves_network);
        pf.initialize_cache(valves_to_precache);
        pf
    }

    pub fn shortest_path_cached(&self, from: ValveId, to: ValveId) -> &[ValveId] {
        self.cache.get(&(from, to)).unwrap()
    }

    fn shortest_path(&mut self, from: ValveId, to: ValveId) -> &[ValveId] {
        let key = (from, to);
        assert!(from != to);

        if self.cache.contains_key(&key) {
            return self.cache.get(&key).unwrap();
        }
        match self.breadth_first_search(from, to) {
            None => unreachable!("Isolated island of valves are not expected!"),
            Some(mut path) => {
                self.cache.insert(key, path.clone());
                let rev_key = (to, from);
                path.reverse();
                self.cache.insert(rev_key, path);
                self.cache.get(&key).unwrap()
            }
        }
    }

    fn breadth_first_search(&self, from: ValveId, to: ValveId) -> Option<Vec<ValveId>> {
        let mut from_queue: Vec<ValveId> = vec![from];
        let mut to_queue: Vec<ValveId> = vec![to];

        let mut from_queue_next: Vec<ValveId> = vec![];
        let mut to_queue_next: Vec<ValveId> = vec![];

        let mut from_lineage = HashMap::new();
        from_lineage.insert(from, vec![from]);

        let mut to_lineage = HashMap::new();
        to_lineage.insert(to, vec![to]);

        while !from_lineage.is_empty() || !to_lineage.is_empty() {
            for from_elem in from_queue {
                for &neighbour in self.valve[from_elem].link_valves_ids.iter() {
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
                for &neighbour in self.valve[to_elem].link_valves_ids.iter() {
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

        // ok, if we reached this line it means that there are isolated isaland of valves,
        // which should not happen for the input data that we have seen but it is possible in theory
        None
    }

    pub fn initialize_cache(&mut self, relevant_valves: &[usize]) {
        for from in relevant_valves.iter() {
            for to in relevant_valves.iter() {
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

fn generate_possible_routes(
    path_finder: &PathFinder,
    initial: ValveId,
    valves_to_open: &[ValveId],
    time_remaining: usize,
) -> Vec<Vec<Action>> {
    let mut ans: Vec<Vec<Action>> = Vec::new();

    for (valve_id, &valve) in valves_to_open.iter().enumerate() {
        let path = path_finder.shortest_path_cached(initial, valve);
        if path.len() >= time_remaining {
            continue;
        }
        let mut actions = Vec::from_iter(path.iter().skip(1).map(|&n| Action::Move(n)));
        actions.push(Action::Open(valve));

        let mut new_valves = Vec::with_capacity(valves_to_open.len() - 1);
        if valve_id > 0 {
            new_valves.append(&mut valves_to_open[..valve_id].to_vec());
        }
        if valve_id < valves_to_open.len() - 1 {
            new_valves.append(&mut valves_to_open[valve_id + 1..].to_vec());
        }

        let next_actions =
            generate_possible_routes(path_finder, valve, &new_valves, time_remaining - path.len());

        // always add a route that preemptivbely stops and does not go further,
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

/// Returns the list of pairs:
/// - the first is the encoded set of all valves that were opened during route
/// - the actual value of the total pressure released
pub fn generate_scenarios_for_single_worker(
    valves: &[Valve],
    time_budget: usize,
    initial_valve_idx: usize,
) -> Vec<(EncodedRouteValvesSet, usize)> {
    let positive_rate_valves = (0..valves.len())
        .filter(|&i| valves[i].rate > 0)
        .collect::<Vec<_>>();

    let path_finder = {
        let mut relevant_valves = positive_rate_valves.clone();
        relevant_valves.push(initial_valve_idx);
        PathFinder::new_with_cache(valves, &relevant_valves)
    };

    generate_possible_routes(
        &path_finder,
        initial_valve_idx,
        &positive_rate_valves,
        time_budget,
    )
    .iter()
    .map(|route| {
        let total_pressure_released = route
            .iter()
            .enumerate()
            .filter_map(|(time, a)| match a {
                Action::Open(valve_id) => Some(valves[*valve_id].rate * (time_budget - time - 1)),
                _ => None,
            })
            .sum();

        // Compactly encodes all valves that we opened during the route into a single value.
        // For example [1, 3, 5] becomes b`10101`.
        // This can be used later for efficiently checking disjoint interections of valve sets.
        let encoded_route_valves_set: EncodedRouteValvesSet = route
            .iter()
            .filter_map(|action| match action {
                Action::Open(valve_id) => Some(valve_id),
                _ => None,
            })
            .fold(0, |init_zero, valve_id| init_zero | (1 << valve_id));
        (encoded_route_valves_set, total_pressure_released)
    })
    .collect()
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
        let valve_network = parse_valves_network(asset.data.as_ref());
        let mut path_finder = PathFinder::new(&valve_network);

        assert_eq!(path_finder.shortest_path(0, 5), vec![0, 3, 4, 5]);
        assert_eq!(path_finder.shortest_path(5, 0), vec![5, 4, 3, 0]);

        assert_eq!(path_finder.shortest_path(6, 4), vec![6, 5, 4]);
        assert_eq!(path_finder.shortest_path(4, 6), vec![4, 5, 6]);

        assert_eq!(path_finder.shortest_path(0, 9), vec![0, 8, 9]);
        assert_eq!(path_finder.shortest_path(9, 0), vec![9, 8, 0]);

        assert_eq!(path_finder.shortest_path_cached(0, 9), vec![0, 8, 9]);
        assert_eq!(path_finder.shortest_path_cached(9, 0), vec![9, 8, 0]);
    }
}
