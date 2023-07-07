#![feature(lazy_cell)]

use regex::Regex;
use std::{
    io::{BufRead, BufReader, Read},
    sync::LazyLock,
};

static CMD_NOOP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"noop").unwrap());
static CMD_ADDX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"addx (?P<value>[-]?\d+)").unwrap());

pub fn x_register_values(read: impl Read) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut x = Vec::with_capacity(256);
    // x[0] and x[1] - initial value set to `1`
    x.push(1);
    x.push(1);

    for line in BufReader::new(read).lines() {
        let cmd_line = &line?;
        if CMD_NOOP.is_match(cmd_line) {
            x.push(*x.last().unwrap());
        } else if CMD_ADDX.is_match(cmd_line) {
            let captures = CMD_ADDX.captures_iter(cmd_line).next().unwrap();
            let value: i32 = captures.name("value").unwrap().as_str().parse().unwrap();
            x.push(*x.last().unwrap());
            x.push(*x.last().unwrap() + value);
        } else {
            unreachable!("Unknown command: {cmd_line}")
        }
    }

    Ok(x)
}
