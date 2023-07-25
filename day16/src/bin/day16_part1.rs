/*! See https://adventofcode.com/2022/day/16 */

#![allow(non_upper_case_globals)]

use day16::{generate_scenarios_for_single_worker, parse_valves_network, ValveId};

use rust_embed::RustEmbed;
use std::io::Read;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn calculate_max_pressure_released(
    reader: impl Read,
    time_budget_mins: usize,
    initial_valve_label: &str,
) -> usize {
    let valves_network = parse_valves_network(reader);
    let initial_valve_id: ValveId = valves_network
        .iter()
        .position(|valve| valve.label.eq(initial_valve_label))
        .unwrap();

    // All the work has already been done,
    // all that left is simply get the max total pressure released
    generate_scenarios_for_single_worker(&valves_network, time_budget_mins, initial_valve_id)
        .into_iter()
        .map(|(_, total_pressure)| total_pressure)
        .max()
        .unwrap()
}

fn main() {
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
