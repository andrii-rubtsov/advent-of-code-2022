/*! See https://adventofcode.com/2022/day/16 */

use day16::Node;
use log::LevelFilter;

use rust_embed::RustEmbed;
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
struct CacheKey {
    current_node_idx: usize,
    open_valves: u128, // ASSUMPTION: the total number of valves is less than 128
    time_budget: usize,
}

fn decode_open_valves_indexes(open_valves: u128) -> HashSet<usize> {
    (0..128)
        .filter(|idx| open_valves & (1 << idx) > 0)
        .collect()
}

impl CacheKey {
    fn new(current_node_idx: usize, open_valves: u128, time_budget: usize) -> Self {
        CacheKey {
            current_node_idx,
            open_valves,
            time_budget,
        }
    }

    pub fn is_valve_open(&self, node_index: usize) -> bool {
        self.open_valves & (1 << node_index) > 0
    }

    fn key_for_opened_node(&self, node_idx: usize) -> Self {
        CacheKey::new(
            node_idx,
            self.open_valves | 1 << node_idx,
            self.time_budget - 1,
        )
    }

    fn key_for_go_to_new_node(&self, new_node_idx: usize) -> Self {
        CacheKey::new(new_node_idx, self.open_valves, self.time_budget - 1)
    }

    fn decode_open_valves_indexes(&self) -> HashSet<usize> {
        decode_open_valves_indexes(self.open_valves)
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Idle,
    Move(usize),
    Open(usize),
}

#[derive(Clone, Debug)]
struct OptimizedResult {
    total_pressure_released: usize,
    actions_lineage: Vec<Action>,
}

impl OptimizedResult {
    pub fn new(total_pressure_released: usize, actions_lineage: Vec<Action>) -> Self {
        OptimizedResult {
            total_pressure_released,
            actions_lineage,
        }
    }
}

struct CachingReleasedPressureOptimizer<'a> {
    pub nodes: &'a Vec<Node>,
    cache: HashMap<CacheKey, OptimizedResult>,
}

impl<'a> CachingReleasedPressureOptimizer<'a> {
    pub fn new(nodes: &'a Vec<Node>) -> Self {
        CachingReleasedPressureOptimizer {
            nodes,
            cache: HashMap::new(),
        }
    }

    /// Calculates the maximum amount of released pressure.
    /// Also returns the lineage of actions (for debugging, if needed)
    /// that lead to the optimal solution.
    pub fn maximize_released_pressure(
        &mut self,
        time_budget_mins: usize,
        initial_node_label: &str,
    ) -> OptimizedResult {
        let initial_node_idx = self
            .nodes
            .iter()
            .position(|node| node.label.eq(initial_node_label))
            .unwrap();

        let cache_key_initial = CacheKey::new(
            initial_node_idx,
            0, // currently opened valves (none)
            time_budget_mins,
        );
        self.maximize_for_cache_key(cache_key_initial)
    }

    fn calc_released_pressure_for_open(nodes: &[Node], cache_key: &CacheKey) -> usize {
        cache_key
            .decode_open_valves_indexes()
            .iter()
            .map(|&idx| nodes[idx].rate)
            .sum()
    }

    fn maximize_for_cache_key(&mut self, cache_key: CacheKey) -> OptimizedResult {
        if let Some(cached) = self.cache.get(&cache_key) {
            return cached.clone();
        }
        let node_idx = cache_key.current_node_idx;

        // No matter what, the currently open valves will release pressure for the given cycle
        let current_total_rate = CachingReleasedPressureOptimizer::calc_released_pressure_for_open(
            self.nodes, &cache_key,
        );

        let (mut next_optimized_result, action): (OptimizedResult, Action) =
            match cache_key.time_budget {
                // if there is only one time cycle left,
                // it is not enough to open or move to a neighbour node
                1 => (OptimizedResult::new(0, vec![]), Action::Idle),
                time => {
                    // 3 actions possible (need to choose the best):

                    // 1) The first option is nothing
                    let mut action = Action::Idle;
                    let mut optimal = OptimizedResult::new(
                        (time - 1) * current_total_rate,
                        vec![Action::Idle; time - 1],
                    );

                    // 2) Second option is to try opening the current valve
                    if self.nodes[node_idx].rate > 0 && !cache_key.is_valve_open(node_idx) {
                        let new_cache_key = cache_key.key_for_opened_node(node_idx);
                        let result = self.maximize_for_cache_key(new_cache_key);
                        if optimal.total_pressure_released < result.total_pressure_released {
                            optimal = result;
                            action = Action::Open(node_idx);
                        }
                    }

                    // 3) The third option is to try going to neighbours
                    let neighbours = self.nodes[node_idx].tunnels_idx.clone();
                    for neighbour_idx in neighbours {
                        let new_cache_key = cache_key.key_for_go_to_new_node(neighbour_idx);
                        let result = self.maximize_for_cache_key(new_cache_key);
                        if optimal.total_pressure_released < result.total_pressure_released {
                            optimal = result;
                            action = Action::Move(neighbour_idx);
                        }
                    }
                    (optimal, action)
                }
            };

        let mut optimal_actions = vec![action];
        optimal_actions.append(&mut next_optimized_result.actions_lineage);

        let updated_result = OptimizedResult::new(
            next_optimized_result.total_pressure_released + current_total_rate,
            optimal_actions,
        );

        self.cache.insert(cache_key, updated_result.clone());
        updated_result
    }
}

fn parse_nodes(reader: impl Read) -> Vec<Node> {
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

fn calculate_max_pressure_released(
    reader: impl Read,
    time_budget_mins: usize,
    initial_node_label: &str,
) -> usize {
    let nodes = parse_nodes(reader);
    let mut optimizer = CachingReleasedPressureOptimizer::new(&nodes);

    let optimized_result =
        optimizer.maximize_released_pressure(time_budget_mins, initial_node_label);

    if log::log_enabled!(log::Level::Debug) {
        let mut open_valves: Vec<usize> = vec![];
        let mut total_rate = 0;
        let mut released_so_far = 0;

        for (idx, &action) in optimized_result.actions_lineage.iter().enumerate() {
            log::debug!("=== Minute {} ===", idx + 1);
            if open_valves.is_empty() {
                log::debug!("Valves open: <none>");
            } else {
                log::debug!(
                    "Open valves: {}",
                    open_valves
                        .iter()
                        .map(|&idx| format!("{}-{}", &nodes[idx].label, idx + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }

            released_so_far += total_rate;
            log::debug!("Releasing: {} pressure", total_rate);
            log::debug!("Total released: {} pressure", released_so_far);

            match action {
                Action::Idle => {
                    log::debug!("Action: idle");
                }
                Action::Move(node_idx) => {
                    log::debug!(
                        "Action: move to node {}-{}",
                        &nodes[node_idx].label,
                        node_idx + 1
                    );
                }
                Action::Open(node_idx) => {
                    open_valves.push(node_idx);
                    total_rate += nodes[node_idx].rate;
                    log::debug!(
                        "Action: open node {}-{}; rate increased by: {}, updated rate: {}",
                        &nodes[node_idx].label,
                        node_idx + 1,
                        &nodes[node_idx].rate,
                        total_rate
                    );
                }
            }
            log::debug!("");
        }
    }

    optimized_result.total_pressure_released
}

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter(None, LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    let asset = Asset::get("input.txt").unwrap();
    let max_pressure_released = calculate_max_pressure_released(asset.data.as_ref(), 30, "AA");
    log::info!("Max pressure released: {max_pressure_released}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let max_pressure_released = calculate_max_pressure_released(asset.data.as_ref(), 30, "AA");
        assert_eq!(max_pressure_released, 1651);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let max_pressure_released = calculate_max_pressure_released(asset.data.as_ref(), 30, "AA");
        assert_eq!(max_pressure_released, 1376);
    }
}
