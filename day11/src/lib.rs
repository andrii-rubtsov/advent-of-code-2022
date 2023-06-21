use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

enum Operand {
    Argument,
    IntValue(u128),
}

enum BinaryOperator {
    Add,
    Multiply,
}

pub struct Operation {
    left_operand: Operand,
    right_operand: Operand,
    binary_operator: BinaryOperator,
}

impl Operation {
    pub fn apply(&self, arg: u128) -> u128 {
        let left = match self.left_operand {
            Operand::Argument => arg,
            Operand::IntValue(val) => val,
        };
        let right = match self.right_operand {
            Operand::Argument => arg,
            Operand::IntValue(val) => val,
        };
        match self.binary_operator {
            BinaryOperator::Add => left + right,
            BinaryOperator::Multiply => left * right,
        }
    }

    pub fn parse(op_str: &str) -> Self {
        let parts: Vec<_> = op_str.split(' ').collect();
        let left_operand: Operand = match parts[0] {
            "old" => Operand::Argument,
            s => Operand::IntValue(u128::from_str(s).unwrap()),
        };

        let right_operand: Operand = match parts[2] {
            "old" => Operand::Argument,
            s => Operand::IntValue(u128::from_str(s).unwrap()),
        };

        let binary_operator: BinaryOperator = match parts[1] {
            "+" => BinaryOperator::Add,
            "*" => BinaryOperator::Multiply,
            _ => panic!("Unexpected binary operator found: {}", parts[1]),
        };

        Operation {
            left_operand,
            right_operand,
            binary_operator,
        }
    }
}

pub struct DivisibleByTest {
    divisor: u128,
    true_idx: usize,
    false_idx: usize,
}

impl DivisibleByTest {
    fn next_monkey_index(&self, stress_level: u128) -> usize {
        match stress_level % self.divisor == 0 {
            true => self.true_idx,
            false => self.false_idx,
        }
    }

    pub fn divisor(&self) -> u128 {
        self.divisor
    }
}

pub struct Monkey {
    pub items: Vec<u128>,
    pub operation: Operation,
    pub divisible_by_test: DivisibleByTest,
    pub total_inspections: usize,
}

impl Monkey {
    // Expects input in thew style of smth like:
    // Monkey 0:
    //   Starting items: 89, 95, 92, 64, 87, 68
    //   Operation: new = old * 11
    //   Test: divisible by 2
    //   If true: throw to monkey 7
    //   If false: throw to monkey 4
    pub fn parse(buffer: &str) -> Monkey {
        let mut starting_items: Vec<u128> = vec![];
        let mut operation: Option<Operation> = Option::None;
        let mut divisor: u128 = u128::default();
        let mut true_idx: usize = 0;
        let mut false_idx: usize = 0;

        fn match_and_strip<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
            let line = line.trim_start();
            let prefix = prefix.trim_start();
            if line.starts_with(prefix) {
                Some(line.strip_prefix(prefix).unwrap())
            } else {
                None
            }
        }

        for line in buffer.lines() {
            if line.is_empty() || line.starts_with("Monkey") {
                continue;
            } else if let Some(remainder) = match_and_strip(line, "Starting items:") {
                starting_items = remainder
                    .split(',')
                    .map(str::trim)
                    .map(|str_item: &str| u128::from_str(str_item).unwrap())
                    .collect();
            } else if let Some(remainder) = match_and_strip(line, "Operation: new = ") {
                operation = Option::Some(Operation::parse(remainder));
            } else if let Some(remainder) = match_and_strip(line, "Test: divisible by ") {
                divisor = str::parse(remainder).unwrap();
            } else if let Some(remainder) = match_and_strip(line, "If true: throw to monkey ") {
                true_idx = str::parse(remainder).unwrap();
            } else if let Some(remainder) = match_and_strip(line, "If false: throw to monkey ") {
                false_idx = str::parse(remainder).unwrap();
            }
        }
        let divisible_by_test = DivisibleByTest {
            divisor,
            true_idx,
            false_idx,
        };

        Monkey {
            items: starting_items,
            operation: operation.unwrap(),
            divisible_by_test,
            total_inspections: 0,
        }
    }
}

pub fn parse_all_monkeys(reader: impl Read) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut buffer = String::new();
    for input_line in BufReader::new(reader).lines() {
        let line = input_line.unwrap();
        if line.is_empty() {
            if !buffer.is_empty() {
                monkeys.push(Monkey::parse(&buffer));
            }
            buffer = String::new();
        } else {
            buffer.push_str(&line);
            buffer.push('\n');
        }
    }
    if !buffer.is_empty() {
        monkeys.push(Monkey::parse(&buffer));
    }
    monkeys
}

pub fn process_round(
    monkeys: &mut Vec<Monkey>,
    decrease_stress_level: bool,
    common_divisor: Option<u128>,
) {
    for i in 0..monkeys.len() {
        let mut new_owners: HashMap<usize, Vec<u128>> = HashMap::new();

        let mut monkey = &mut monkeys[i];
        for &stress_level in &monkey.items {
            let mut new_stress_level = monkey.operation.apply(stress_level);
            if decrease_stress_level {
                new_stress_level /= 3;
            }

            // Critical optimization: assuming all tests are "divisible by", it is mathematically ok
            // to use the remainder after division by common divisor
            if let Some(max) = common_divisor {
                if new_stress_level >= max {
                    new_stress_level %= max;
                }
            }

            let next_monkey_idx = monkey.divisible_by_test.next_monkey_index(new_stress_level);

            new_owners
                .entry(next_monkey_idx)
                .or_default()
                .push(new_stress_level);
        }
        monkey.total_inspections += monkey.items.len();
        monkey.items.clear();

        for (idx, mut new_items) in new_owners {
            monkeys[idx].items.append(&mut new_items);
        }
    }
}

pub fn calculate_monkey_business(monkeys: &mut [Monkey]) -> u128 {
    monkeys.sort_by_key(|m| -(m.total_inspections as i128));
    monkeys
        .iter()
        .take(2)
        .map(|m| m.total_inspections as u128)
        .reduce(|acc, e| acc * e)
        .unwrap()
}
