enum Operand {
    Argument,
    IntValue(u32),
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
    pub fn apply(&self, arg: u32) -> u32 {
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
            s => Operand::IntValue(str::parse::<u32>(s).unwrap()),
        };

        let right_operand: Operand = match parts[2] {
            "old" => Operand::Argument,
            s => Operand::IntValue(str::parse::<u32>(s).unwrap()),
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

pub trait NextMonkey {
    fn next_monkey_index(&self, stress_level: u32) -> usize;
}

pub struct DivisibleByNextMonkeyTest {
    divisor: u32,
    true_idx: usize,
    false_idx: usize,
}

impl NextMonkey for DivisibleByNextMonkeyTest {
    fn next_monkey_index(&self, stress_level: u32) -> usize {
        match stress_level % self.divisor == 0 {
            true => self.true_idx,
            false => self.false_idx,
        }
    }
}

pub struct Monkey {
    pub items: Vec<u32>,
    pub operation: Operation,
    pub next_monkey: Box<dyn NextMonkey>,
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
        let mut starting_items: Vec<u32> = vec![];
        let mut operation: Option<Operation> = Option::None;
        let mut divisor: u32 = 0;
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
                    .map(|str_item: &str| str::parse::<u32>(str_item).unwrap())
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
        let next_monkey = DivisibleByNextMonkeyTest {
            divisor,
            true_idx,
            false_idx,
        };

        Monkey {
            items: starting_items,
            operation: operation.unwrap(),
            next_monkey: Box::new(next_monkey),
            total_inspections: 0,
        }
    }
}
