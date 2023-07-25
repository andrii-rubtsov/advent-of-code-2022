/*! See https://adventofcode.com/2022/day/16 */

#![allow(non_upper_case_globals)]

use day16::{
    calc_pressure_released, generate_possible_routes, parse_nodes, Action, Node, OptimalResult,
    PathFinder,
};
use log::LevelFilter;

use rust_embed::RustEmbed;
use std::{collections::HashSet, io::Read};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

struct ParallelReleasedPressureOptimizer {
    pub nodes: Vec<Node>,
}

impl ParallelReleasedPressureOptimizer {
    pub fn new(nodes: Vec<Node>) -> Self {
        ParallelReleasedPressureOptimizer { nodes }
    }

    /// Calculates the maximum amount of released pressure.
    /// Also returns the lineage of actions (for debugging, if needed)
    /// that lead to the optimal solution.
    pub fn maximize_released_pressure(
        &mut self,
        time_budget: usize,
        initial_node_label: &str,
    ) -> OptimalResult<(Action, Action)> {
        let initial_node_idx = self
            .nodes
            .iter()
            .position(|node| node.label.eq(initial_node_label))
            .unwrap();

        let positive_rate_nodes = (0..self.nodes.len())
            .filter(|&i| self.nodes[i].rate > 0)
            .collect::<Vec<_>>();
        let positive_rate_nodes_set = HashSet::from_iter(positive_rate_nodes.iter().cloned());

        let path_finder = {
            let mut relevant_nodes = positive_rate_nodes.clone();
            relevant_nodes.push(initial_node_idx);
            PathFinder::new_with_cache(&self.nodes, &relevant_nodes)
        };

        let own_routes = generate_possible_routes(
            &path_finder,
            initial_node_idx,
            &positive_rate_nodes,
            time_budget,
        );

        let mut max_pressure_released = 0;
        let mut lineage: Vec<(Action, Action)> = Vec::new();
        for own_route in own_routes.iter() {
            let total_own_released = calc_pressure_released(own_route, &self.nodes, time_budget);

            let remaining_nodes_to_open: Vec<usize> = {
                let own_route_open_nodes: HashSet<usize> = own_route
                    .iter()
                    .filter_map(|a| match a {
                        Action::Open(node_id) => Some(*node_id),
                        _ => None,
                    })
                    .collect();
                positive_rate_nodes_set
                    .difference(&own_route_open_nodes)
                    .cloned()
                    .collect()
            };
            let elephant_routes = generate_possible_routes(
                &path_finder,
                initial_node_idx,
                &remaining_nodes_to_open,
                time_budget,
            );
            if elephant_routes.is_empty() {
                continue; // we can just rule these out because they will not be the most efficient
            }

            for elephant_route in elephant_routes.iter() {
                let total_elephant_released =
                    calc_pressure_released(elephant_route, &self.nodes, time_budget);
                if max_pressure_released < total_own_released.0 + total_elephant_released.0 {
                    max_pressure_released = total_own_released.0 + total_elephant_released.0;
                    lineage = Vec::from_iter(
                        total_own_released
                            .1
                            .iter()
                            .cloned()
                            .zip(total_elephant_released.1),
                    );
                }
            }
        }
        OptimalResult::new(max_pressure_released, lineage)
    }

    pub fn print_lineage(
        &self,
        result: &OptimalResult<(Action, Action)>,
        teaching_elephant_time_budget_mins: usize,
    ) {
        let mut open_valves: Vec<usize> = vec![];
        let mut total_rate = 0;
        let mut released_so_far = 0;

        for min in 1..(teaching_elephant_time_budget_mins + 1) {
            log::debug!("=== Minute {} ===", min);
            log::debug!("Own action: teach elephant");
            log::debug!("");
        }

        for (idx, (own_action, elephant_action)) in result.actions_lineage.iter().enumerate() {
            let minute = idx + teaching_elephant_time_budget_mins + 1;
            log::debug!("=== Minute {} ===", minute);
            if open_valves.is_empty() {
                log::debug!("Valves open: <none>");
            } else {
                log::debug!(
                    "Open valves: {}",
                    open_valves
                        .iter()
                        .map(|&idx| format!("{}-{}", &self.nodes[idx].label, idx + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }

            released_so_far += total_rate;
            log::debug!("Releasing: {} pressure", total_rate);
            log::debug!("Total released: {} pressure", released_so_far);

            for (action, is_own_action) in [(own_action, true), (elephant_action, false)] {
                let name = if is_own_action { "Own" } else { "Elephant" };
                match action {
                    Action::Idle => {
                        log::debug!("{} action: idle", name);
                    }
                    Action::Move(node_idx) => {
                        log::debug!(
                            "{} action: move to node {}-{}",
                            name,
                            &self.nodes[*node_idx].label,
                            node_idx + 1
                        );
                    }
                    Action::Open(node_idx) => {
                        open_valves.push(*node_idx);
                        total_rate += self.nodes[*node_idx].rate;
                        log::debug!(
                            "{} action: open node {}-{}; rate increased by: {}, updated rate: {}",
                            name,
                            &self.nodes[*node_idx].label,
                            node_idx + 1,
                            &self.nodes[*node_idx].rate,
                            total_rate
                        );
                    }
                }
            }
            log::debug!("");
        }
    }
}

fn calculate_max_pressure_released(
    reader: impl Read,
    time_budget_mins: usize,
    teaching_elephant_time_budget_mins: usize,
    initial_node_label: &str,
) -> usize {
    let nodes = parse_nodes(reader);
    let mut optimizer = ParallelReleasedPressureOptimizer::new(nodes);

    let optimal_result = optimizer.maximize_released_pressure(
        time_budget_mins - teaching_elephant_time_budget_mins,
        initial_node_label,
    );

    if log::log_enabled!(log::Level::Debug) {
        optimizer.print_lineage(&optimal_result, teaching_elephant_time_budget_mins);
    }

    optimal_result.total_pressure_released
}

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter(None, LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    let asset = Asset::get("input.txt").unwrap();
    let max_pressure_released = calculate_max_pressure_released(asset.data.as_ref(), 30, 4, "AA");
    log::info!("Max pressure released: {max_pressure_released}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let max_pressure_released =
            calculate_max_pressure_released(asset.data.as_ref(), 30, 4, "AA");
        assert_eq!(max_pressure_released, 1707);
    }

    //#[test]
    // Currently it takes ~1 minute to run (probably the solution it suboptimal)
    // Thus it is disabled.
    #[allow(dead_code)]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let max_pressure_released =
            calculate_max_pressure_released(asset.data.as_ref(), 30, 4, "AA");
        assert_eq!(max_pressure_released, 1933);
    }
}
