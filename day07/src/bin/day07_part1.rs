/*! See https://adventofcode.com/2022/day/7 */

#![allow(non_upper_case_globals)]

use day07::{build_virtual_fs, Node};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "."]
pub struct Asset;

fn sum_dir_sizes_below_limit(root: &Node, dir_size_limit: usize) -> usize {
    let mut total = 0;
    let own_size = root.total_size();
    if own_size <= dir_size_limit {
        total += own_size;
    }
    total += root
        .iter_directories()
        .map(|dir| sum_dir_sizes_below_limit(&dir, dir_size_limit))
        .sum::<usize>();

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Asset::get("input.txt").unwrap();
    let virtual_fs = build_virtual_fs(input.data.as_ref())?;
    let total_dir_size = sum_dir_sizes_below_limit(&virtual_fs, 100_000);
    println!("Dir size sum: {total_dir_size}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn test_sum_dir_sizes_below_limit() {
        let root = Rc::new(RefCell::new(Node::new_directory("/".into())));
        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("1".into(), 10))));
        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("2".into(), 5))));
        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("3".into(), 5))));

        let child1 = Rc::new(RefCell::new(Node::new_directory("child1".into())));
        root.borrow_mut().add_node(Rc::clone(&child1));

        child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("10".into(), 1))));
        child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("20".into(), 2))));
        child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("30".into(), 3))));

        let child1_child1 = Rc::new(RefCell::new(Node::new_directory("child1_child1".into())));
        child1.borrow_mut().add_node(Rc::clone(&child1_child1));

        child1_child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("10".into(), 1))));
        child1_child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("20".into(), 2))));
        child1_child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("30".into(), 3))));

        let root_node = Rc::try_unwrap(root).ok().unwrap().into_inner();

        assert_eq!(sum_dir_sizes_below_limit(&root_node, 10), 6);
    }

    #[test]
    fn test_test_input() {
        let input = Asset::get("test_input.txt").unwrap();
        let virtual_fs = build_virtual_fs(input.data.as_ref()).unwrap();
        let total_dir_size = sum_dir_sizes_below_limit(&virtual_fs, 100_000);
        assert_eq!(total_dir_size, 95437);
    }

    #[test]
    fn test_actual_input() {
        let input = Asset::get("input.txt").unwrap();
        let virtual_fs = build_virtual_fs(input.data.as_ref()).unwrap();
        let total_dir_size = sum_dir_sizes_below_limit(&virtual_fs, 100_000);
        assert_eq!(total_dir_size, 1770595);
    }
}
