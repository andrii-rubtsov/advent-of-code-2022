/*! See https://adventofcode.com/2022/day/16 */

#![allow(non_upper_case_globals)]

use day16::{generate_scenarios_for_single_worker, parse_valves_network, ValveId};

use rust_embed::RustEmbed;
use std::{cmp::Reverse, io::Read};

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn calculate_max_pressure_released(
    reader: impl Read,
    time_budget_mins: usize,
    teaching_elephant_time_budget_mins: usize,
    initial_valve_label: &str,
) -> usize {
    let valves_network = parse_valves_network(reader);
    let initial_valve_id: ValveId = valves_network
        .iter()
        .position(|valve| valve.label.eq(initial_valve_label))
        .unwrap();

    let mut singe_worker_results = generate_scenarios_for_single_worker(
        &valves_network,
        time_budget_mins - teaching_elephant_time_budget_mins,
        initial_valve_id,
    );

    singe_worker_results.sort_unstable_by_key(|(_, pressure)| Reverse(*pressure));

    // The idea is that it is safe to assume human and elephant are acting totally independently.
    // The only condition is that the sets of open valves should be disjoint.
    // The double loop below is optimized to quickly find a pair of results that maximizes
    // the total pressure released from independtent runs. Naiive approach is too slow,
    // because all combinations count as many as 1.5 billions pairs
    // (takes more than 20 seconds without `release` optimizations)
    let mut curr_max = 0;
    for i in 0..singe_worker_results.len() - 1 {
        for j in i..singe_worker_results.len() {
            let (encodned_valves_set_i, pressure_i) = singe_worker_results[i];
            let (encodned_valves_set_j, pressure_j) = singe_worker_results[j];
            let common_pressure = pressure_i + pressure_j;
            if encodned_valves_set_i & encodned_valves_set_j == 0 && curr_max < common_pressure {
                curr_max = common_pressure;
            } else if common_pressure < curr_max {
                // break inner loop preemptively, because the sums are only getting smaller
                // within the innermost loop and it can no longer beet current max
                // (even if more disjoin sets found eventually)
                break;
            }
        }
    }
    curr_max
}

fn main() {
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

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let max_pressure_released =
            calculate_max_pressure_released(asset.data.as_ref(), 30, 4, "AA");
        assert_eq!(max_pressure_released, 1933);
    }
}
