/*! See https://adventofcode.com/2022/day/7 */

use day7::{build_virtual_fs, Node};
use log::LevelFilter;
use pretty_env_logger::env_logger::{Builder, WriteStyle};

fn smallest_dir_size(root: &Node, min_dir_size: usize) -> Option<usize> {
    root.iter_directories()
        .filter_map(|dir| smallest_dir_size(&dir, min_dir_size))
        .min()
        .or(Some(root.total_size()).filter(|&s| s >= min_dir_size))
}

fn get_smallest_dir_size_to_delete() -> Result<usize, Box<dyn std::error::Error>> {
    let virtual_fs = build_virtual_fs("input.txt")?;
    let total_disk_size: usize = 70_000_000;
    let required_for_update_size: usize = 30_000_000;
    let total_fs_size_limit = total_disk_size - required_for_update_size;
    let curr_total_fs_size = virtual_fs.total_size();

    let minimal_size_to_free = curr_total_fs_size - total_fs_size_limit;
    assert!(
        minimal_size_to_free > 0,
        "Current size is already enough for update. No cleanup needed."
    );
    Ok(smallest_dir_size(&virtual_fs, minimal_size_to_free).unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .format_timestamp_millis()
        .init();

    let total_dir_size = get_smallest_dir_size_to_delete()?;
    println!("Smallest dir to be deleted: {total_dir_size}");
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum_dir_sizes_below_limit() {
        assert_eq!(get_smallest_dir_size_to_delete().unwrap(), 2195372);
    }
}
