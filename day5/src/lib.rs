use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
    static ref COMMAND_REGEX: Regex =
        Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
}

#[derive(Debug)]
pub struct CrateStacks {
    stacks: Vec<Vec<char>>, // 0-based
}

impl CrateStacks {
    // Parses [CrateStacks] from the folloing text representation:
    //
    //            [G]         [D]     [Q]
    //    [P]     [T]         [L] [M] [Z]
    //    [Z] [Z] [C]         [Z] [G] [W]
    //    [M] [B] [F]         [P] [C] [H] [N]
    //    [T] [S] [R]     [H] [W] [R] [L] [W]
    //    [R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
    //    [C] [N] [H] [R] [N] [H] [D] [J] [Q]
    //    [N] [D] [M] [G] [Z] [F] [W] [S] [S]
    //     1   2   3   4   5   6   7   8   9
    pub fn from_text_repr(mut stacks_text: Vec<&str>) -> CrateStacks {
        let index_line = stacks_text.pop().unwrap();
        stacks_text.reverse();
        let mut stacks_vec = Vec::with_capacity(10);
        for num_match in NUMBER_REGEX.find_iter(index_line) {
            let num = num_match.as_str();
            let num_idx = index_line.find(num).unwrap();
            let mut v = vec![];
            for line in stacks_text.iter() {
                if num_idx < line.len() {
                    let c = line.as_bytes()[num_idx] as char;
                    if c.is_ascii_alphabetic() {
                        v.push(c);
                    }
                }
            }
            stacks_vec.push(v);
        }
        CrateStacks { stacks: stacks_vec }
    }

    pub fn apply(&mut self, command: &Command) {
        for _ in 0..(command.amount) {
            if let Some(item) = self.stacks.get_mut(command.from - 1).unwrap().pop() {
                let target_stack = self.stacks.get_mut(command.to - 1).unwrap();
                target_stack.push(item);
            } else {
                eprintln!("Unable to execute command: {:?}. Source is empty", command);
            }
        }
    }

    pub fn apply_preserve_order(&mut self, command: &Command) {
        let drained: Vec<_> = {
            let source_stack = self.stacks.get_mut(command.from - 1).unwrap();
            let start_idx = source_stack.len() - command.amount;
            source_stack.drain(start_idx..).collect()
        };
        if drained.len() != command.amount {
            panic!(
                "Unable to execute command `{:?}`: not enough items in the source stack",
                drained
            )
        }
        let target_stack = self.stacks.get_mut(command.to - 1).unwrap();
        target_stack.extend(drained);
    }

    pub fn peek_top_letters(&self) -> Vec<Option<&char>> {
        self.stacks.iter().map(|vec| vec.last()).collect()
    }
}

#[derive(Debug)]
pub struct Command {
    pub from: usize,
    pub to: usize,
    pub amount: usize,
}

impl From<&str> for Command {
    /// Convert smth like line below into a [Command]
    ///
    /// move 7 from 6 to 8
    fn from(value_str: &str) -> Self {
        let captures = COMMAND_REGEX.captures_iter(value_str).next().unwrap();
        let amount: usize = captures.name("amount").unwrap().as_str().parse().unwrap();
        let from: usize = captures.name("from").unwrap().as_str().parse().unwrap();
        let to: usize = captures.name("to").unwrap().as_str().parse().unwrap();
        Command { from, to, amount }
    }
}
