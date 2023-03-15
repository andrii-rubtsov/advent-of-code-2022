/*! See https://adventofcode.com/2022/day/7 */

use core::panic;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use lazy_static::lazy_static;
use regex::Regex;
use rust_embed::RustEmbed;

lazy_static! {
    static ref CD_CMD: Regex = Regex::new(r"\$\scd\s+(?P<dir>\S+)").unwrap();
    static ref LS_CMD: Regex = Regex::new(r"\$\s+ls").unwrap();
    static ref LS_DIR: Regex = Regex::new(r"dir\s+(?P<dir>\S+)").unwrap();
    static ref LS_FILE: Regex = Regex::new(r"(?P<size>\d+)\s+(?P<name>\S+)").unwrap();
}

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

#[derive(Debug)]
pub enum Node {
    Directory {
        name: String,
        nodes: Vec<Rc<RefCell<Node>>>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl Node {
    pub fn new_file(name: String, size: usize) -> Self {
        Node::File { name, size }
    }

    pub fn new_directory(name: String) -> Self {
        Node::Directory {
            name,
            nodes: vec![],
        }
    }

    pub fn add_node(&mut self, node: Rc<RefCell<Node>>) {
        match self {
            Node::File { name: _, size: _ } => panic!("Cannot add nodes to leaf nodes"),
            Node::Directory { name: _, nodes } => nodes.push(node),
        }
    }

    pub fn total_size(&self) -> usize {
        match self {
            Node::File { name: _, size } => *size,
            Node::Directory { name: _, nodes } => {
                nodes.iter().map(|elem| elem.borrow().total_size()).sum()
            }
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Node::File { name: _, size: _ })
    }

    pub fn is_directory(&self) -> bool {
        !self.is_file()
    }

    pub fn iter_directories(&self) -> impl Iterator<Item = Ref<Node>> {
        match self {
            Node::File { name: _, size: _ } => panic!("Cannot iterator over leaf nodes"),
            Node::Directory { name: _, nodes } => nodes
                .iter()
                .filter(|&node| node.as_ref().borrow().is_directory())
                .map(|node| node.as_ref().borrow()),
        }
    }
}

fn build_virtual_fs(file_path: &str) -> Result<Node, Box<dyn std::error::Error>> {
    let input_resource = Asset::get(file_path).unwrap();
    let input_string = std::str::from_utf8(input_resource.data.as_ref())?;

    let mut dir_chain: Vec<Rc<RefCell<Node>>> = vec![];

    let mut is_ls_mode = false;

    for line in input_string.lines() {
        if is_ls_mode && line.starts_with("$ ") {
            is_ls_mode = false;
        }
        if is_ls_mode {
            if LS_DIR.is_match(line) {
                let captures = LS_DIR.captures_iter(line).next().unwrap();
                let dir: &str = captures.name("dir").unwrap().as_str();
                let last = dir_chain.last().unwrap();
                let dir_node = Rc::new(RefCell::new(Node::new_directory(dir.into())));
                last.borrow_mut().add_node(dir_node);
            } else if LS_FILE.is_match(line) {
                let captures = LS_FILE.captures_iter(line).next().unwrap();
                let file: &str = captures.name("name").unwrap().as_str();
                let size: usize = captures.name("size").unwrap().as_str().parse().unwrap();
                let last = dir_chain.last().unwrap();
                let dir_node = Rc::new(RefCell::new(Node::new_file(file.into(), size)));
                last.borrow_mut().add_node(dir_node);
            } else {
                panic!("A line under `ls` seems neither a dir nor a file. Line: {line}")
            }
        } else if CD_CMD.is_match(line) {
            let captures = CD_CMD.captures_iter(line).next().unwrap();
            let dir: &str = captures.name("dir").unwrap().as_str();
            if dir == "/" {
                let root = Node::new_directory("/".into());
                dir_chain.push(Rc::new(RefCell::new(root)));
            } else if dir == ".." {
                dir_chain.remove(dir_chain.len() - 1);
            } else {
                let dir_node = Rc::new(RefCell::new(Node::new_directory(dir.into())));
                dir_chain
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .add_node(Rc::clone(&dir_node));
                dir_chain.push(Rc::clone(&dir_node));
            }
        } else if LS_CMD.is_match(line) {
            is_ls_mode = true;
        }
    }

    Ok(Rc::try_unwrap(dir_chain.remove(0))
        .ok()
        .unwrap()
        .into_inner())
}

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
    let virtual_fs = build_virtual_fs("input.txt")?;
    let total_dir_size = sum_dir_sizes_below_limit(&virtual_fs, 100_000);
    println!("Dir size sum: {total_dir_size}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_size_files_only() {
        let root = Rc::new(RefCell::new(Node::new_directory("/".into())));

        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("1".into(), 1))));
        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("2".into(), 2))));
        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("3".into(), 3))));

        assert_eq!(root.borrow().total_size(), 6);
    }

    #[test]
    fn test_total_size_dirs_and_files() {
        let root = Rc::new(RefCell::new(Node::new_directory("/".into())));
        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("1".into(), 1))));
        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("2".into(), 2))));
        root.borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("3".into(), 3))));

        let child1 = Rc::new(RefCell::new(Node::new_directory("child1".into())));
        root.borrow_mut().add_node(Rc::clone(&child1));

        child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("10".into(), 10))));
        child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("20".into(), 20))));
        child1
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(Node::new_file("30".into(), 30))));

        assert_eq!(root.borrow().total_size(), 66);
    }

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
        let virtual_fs = build_virtual_fs("test_input.txt").unwrap();
        let total_dir_size = sum_dir_sizes_below_limit(&virtual_fs, 100_000);
        assert_eq!(total_dir_size, 95437);
    }

    #[test]
    fn test_actual_input() {
        let virtual_fs = build_virtual_fs("input.txt").unwrap();
        let total_dir_size = sum_dir_sizes_below_limit(&virtual_fs, 100_000);
        assert_eq!(total_dir_size, 1770595);
    }
}
