use std::{
    env,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DAY_DIR_NAME: Regex = Regex::new(r"^day\d+$").unwrap();
}

pub fn find_empirically<P>(filename: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let pseudo_relative_target = filename.as_ref();

    let first_component_name = pseudo_relative_target
        .components()
        .next()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
    if !DAY_DIR_NAME.is_match(first_component_name) {
        panic!(
            "The first component must start with `day` followed by day number, but got {:?}",
            pseudo_relative_target
        )
    }

    let current_dir = env::current_dir().unwrap();
    let current_dir_last_component = current_dir
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();

    let mut target_path = PathBuf::new();
    target_path.push(&current_dir);
    if DAY_DIR_NAME.is_match(current_dir_last_component) {
        // if called from one of `dayXX` dirs, go one level up
        target_path.push("..");
    }
    target_path.push(pseudo_relative_target);

    if !target_path.exists() {
        panic!("Resolved file does not exist: {:?}", target_path)
    }
    target_path
}
