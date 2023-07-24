/*! See https://adventofcode.com/2022/day/16 */

use day16::{decode_open_valves_indexes, parse_nodes, Action, Node};
use log::LevelFilter;

use rust_embed::RustEmbed;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
struct CacheKey {
    own_node_idx: usize,
    elephant_node_idx: usize,
    open_valves: u128, // ASSUMPTION: the total number of valves is less than 128
    time_budget: usize,
}

impl CacheKey {
    fn new(
        own_node_idx: usize,
        elephant_node_idx: usize,
        open_valves: u128,
        time_budget: usize,
    ) -> Self {
        CacheKey {
            own_node_idx,
            elephant_node_idx,
            open_valves,
            time_budget,
        }
    }

    pub fn is_valve_open(&self, node_index: usize) -> bool {
        self.open_valves & (1 << node_index) > 0
    }

    fn open(&self, node_idx: usize) -> Self {
        CacheKey {
            open_valves: self.open_valves | 1 << node_idx,
            ..*self
        }
    }

    fn own_move(&self, own_node_idx: usize) -> Self {
        CacheKey {
            own_node_idx,
            ..*self
        }
    }

    fn elephant_move(&self, elephant_node_idx: usize) -> Self {
        CacheKey {
            elephant_node_idx,
            ..*self
        }
    }

    fn time_tick(&self) -> Self {
        CacheKey {
            time_budget: self.time_budget - 1,
            ..*self
        }
    }

    fn decode_open_valves_indexes(&self) -> HashSet<usize> {
        decode_open_valves_indexes(self.open_valves)
    }
}

#[derive(Clone, Debug, Default)]
struct OptimalResult {
    total_pressure_released: usize,
    actions_lineage: Vec<(Action, Action)>,
}

impl OptimalResult {
    pub fn new(total_pressure_released: usize, actions_lineage: Vec<(Action, Action)>) -> Self {
        OptimalResult {
            total_pressure_released,
            actions_lineage,
        }
    }
}

struct CachingReleasedPressureOptimizer<'a> {
    pub nodes: &'a Vec<Node>,
    cache: HashMap<CacheKey, OptimalResult>,
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
    ) -> OptimalResult {
        let initial_node_idx = self
            .nodes
            .iter()
            .position(|node| node.label.eq(initial_node_label))
            .unwrap();

        let cache_key_initial = CacheKey::new(
            initial_node_idx, // own pos
            initial_node_idx, // elephant pos
            0,                // currently opened valves (none)
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

    fn maximize_for_cache_key(&mut self, cache_key: CacheKey) -> OptimalResult {
        use Action::*;
        if let Some(cached) = self.cache.get(&cache_key) {
            return cached.clone();
        }
        let time = cache_key.time_budget;

        let node_idx = cache_key.own_node_idx;
        let elephant_node_idx = cache_key.elephant_node_idx;

        let current_total_rate = CachingReleasedPressureOptimizer::calc_released_pressure_for_open(
            self.nodes, &cache_key,
        );

        let (mut next_optimal_result, next_optimal_actions): (OptimalResult, (Action, Action)) = {
            if time == 1 {
                return OptimalResult::new(current_total_rate, vec![]);
            }
            let mut own_actions = vec![Action::Idle];
            if self.nodes[node_idx].rate > 0 && !cache_key.is_valve_open(node_idx) {
                own_actions.push(Action::Open(node_idx));
            }
            for neighbour_idx in self.nodes[node_idx].tunnels_idx.clone() {
                own_actions.push(Action::Move(neighbour_idx));
            }

            let mut elephant_actions = vec![Action::Idle];
            if self.nodes[elephant_node_idx].rate > 0 && !cache_key.is_valve_open(elephant_node_idx)
            {
                elephant_actions.push(Action::Open(elephant_node_idx));
            }
            for neighbour_idx in self.nodes[elephant_node_idx].tunnels_idx.clone() {
                elephant_actions.push(Action::Move(neighbour_idx));
            }

            let (mut optimal_result, mut optimal_actions) = (
                OptimalResult::new(
                    (time - 1) * current_total_rate,
                    vec![(Action::Idle, Action::Idle); time - 1],
                ),
                (Action::Idle, Action::Idle),
            );
            let new_cache_key = cache_key.time_tick();
            for &own_action in own_actions.iter() {
                for &elephant_action in elephant_actions.iter() {
                    let result = match (own_action, elephant_action) {
                        (Idle, Idle) => OptimalResult::new(
                            (time - 1) * current_total_rate,
                            vec![(Action::Idle, Action::Idle); time - 1],
                        ),
                        (Idle, Open(elephant_node_idx)) => {
                            self.maximize_for_cache_key(new_cache_key.open(elephant_node_idx))
                        }
                        (Idle, Move(elephant_neighbour_node_idx)) => self.maximize_for_cache_key(
                            new_cache_key.elephant_move(elephant_neighbour_node_idx),
                        ),
                        (Move(neighbour_node_idx), Idle) => {
                            self.maximize_for_cache_key(new_cache_key.own_move(neighbour_node_idx))
                        }
                        (Open(own_node_idx), Idle) => {
                            self.maximize_for_cache_key(new_cache_key.open(own_node_idx))
                        }
                        (Open(own_node_idx), Open(elephant_node_idx)) => self
                            .maximize_for_cache_key(
                                new_cache_key.open(own_node_idx).open(elephant_node_idx),
                            ),
                        (Move(neighbour_node_idx), Open(elephant_node_idx)) => self
                            .maximize_for_cache_key(
                                new_cache_key
                                    .own_move(neighbour_node_idx)
                                    .open(elephant_node_idx),
                            ),
                        (Open(own_node_idx), Move(elephant_neighbour_node_idx)) => self
                            .maximize_for_cache_key(
                                new_cache_key
                                    .open(own_node_idx)
                                    .elephant_move(elephant_neighbour_node_idx),
                            ),
                        (Move(neighbour_node_idx), Move(elephant_neighbour_node_idx)) => self
                            .maximize_for_cache_key(
                                new_cache_key
                                    .own_move(neighbour_node_idx)
                                    .elephant_move(elephant_neighbour_node_idx),
                            ),
                    };

                    if result.total_pressure_released > optimal_result.total_pressure_released {
                        optimal_result = result;
                        optimal_actions = (own_action, elephant_action);
                    }
                }
            }
            (optimal_result, optimal_actions)
        };

        let mut optimal_actions = vec![next_optimal_actions];
        optimal_actions.append(&mut next_optimal_result.actions_lineage);

        let updated_result = OptimalResult::new(
            next_optimal_result.total_pressure_released + current_total_rate,
            optimal_actions,
        );

        self.cache.insert(cache_key, updated_result.clone());
        updated_result
    }
}

fn calculate_max_pressure_released(
    reader: impl Read,
    time_budget_mins: usize,
    initial_node_label: &str,
) -> usize {
    let nodes = parse_nodes(reader);
    let mut optimizer = CachingReleasedPressureOptimizer::new(&nodes);

    let optimal_result = optimizer.maximize_released_pressure(time_budget_mins, initial_node_label);

    if log::log_enabled!(log::Level::Debug) {
        print_debug_actions(optimal_result.actions_lineage.iter(), &nodes);
    }

    optimal_result.total_pressure_released
}

fn print_debug_actions<'a>(
    actions_lineage: impl Iterator<Item = &'a (Action, Action)>,
    nodes: &[Node],
) {
    let mut open_valves: Vec<usize> = vec![];
    let mut total_rate = 0;
    let mut released_so_far = 0;

    for (idx, (own_action, elephant_action)) in actions_lineage.enumerate() {
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

        match own_action {
            Action::Idle => {
                log::debug!("Own action: idle");
            }
            Action::Move(node_idx) => {
                log::debug!(
                    "Own action: move to node {}-{}",
                    &nodes[*node_idx].label,
                    node_idx + 1
                );
            }
            Action::Open(node_idx) => {
                open_valves.push(*node_idx);
                total_rate += nodes[*node_idx].rate;
                log::debug!(
                    "Own action: open node {}-{}; rate increased by: {}, updated rate: {}",
                    &nodes[*node_idx].label,
                    node_idx + 1,
                    &nodes[*node_idx].rate,
                    total_rate
                );
            }
        }
        match elephant_action {
            Action::Idle => {
                log::debug!("Elephant action: idle");
            }
            Action::Move(node_idx) => {
                log::debug!(
                    "Elephant action: move to node {}-{}",
                    &nodes[*node_idx].label,
                    node_idx + 1
                );
            }
            Action::Open(node_idx) => {
                open_valves.push(*node_idx);
                total_rate += nodes[*node_idx].rate;
                log::debug!(
                    "Elephant action: open node {}-{}; rate increased by: {}, updated rate: {}",
                    &nodes[*node_idx].label,
                    node_idx + 1,
                    &nodes[*node_idx].rate,
                    total_rate
                );
            }
        }
        log::debug!("");
    }
}

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter(None, LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    let asset = Asset::get("test_input.txt").unwrap();
    let max_pressure_released = calculate_max_pressure_released(asset.data.as_ref(), 26, "AA");
    log::info!("Max pressure released: {max_pressure_released}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let max_pressure_released =
            calculate_max_pressure_released(asset.data.as_ref(), 26, "AA");
        assert_eq!(max_pressure_released, 1707);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let max_pressure_released = calculate_max_pressure_released(asset.data.as_ref(), 26, "AA");
        assert_eq!(max_pressure_released, 0);
    }
}
