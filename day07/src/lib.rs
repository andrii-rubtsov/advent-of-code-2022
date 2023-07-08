#![feature(lazy_cell)]

use core::panic;
use std::{
    cell::{Ref, RefCell},
    io::{BufRead, BufReader, Read},
    rc::Rc,
    sync::LazyLock,
};

use regex::Regex;

static CD_CMD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$\scd\s+(?P<dir>\S+)").unwrap());
static LS_CMD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$\s+ls").unwrap());
static LS_DIR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"dir\s+(?P<dir>\S+)").unwrap());
static LS_FILE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<size>\d+)\s+(?P<name>\S+)").unwrap());

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

pub fn build_virtual_fs(reader: impl Read) -> Result<Node, Box<dyn std::error::Error>> {
    let mut dir_chain: Vec<Rc<RefCell<Node>>> = vec![];

    let mut is_ls_mode = false;

    for maybe_line in BufReader::new(reader).lines() {
        let line_string = maybe_line?;
        let line = line_string.as_str();
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

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
}
