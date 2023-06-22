use pest::{iterators::Pair, Parser};
use std::{cmp::Ordering, fmt, io::Read};

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar_inline = r#"
number = { ASCII_DIGIT+ }
list = { "[" ~ (number ~ (",")? | list ~ (",")?)* ~ "]" }
line_separator = _{ ("\r\n" | "\n") }
packet_pair = {list ~ line_separator ~ list }
file = { SOI ~ (packet_pair) ~ (line_separator{2} ~ packet_pair)* ~ EOI }
"#]
pub struct PacketParser;

#[derive(Eq, Clone)]
pub enum Elem {
    Number(usize),
    List(Vec<Elem>),
}

pub fn compare_elems(a: &Elem, b: &Elem) -> Ordering {
    use Elem::*;
    match (a, b) {
        (Number(first_num), Number(second_num)) => first_num.cmp(second_num),
        (Number(num), List(_)) => compare_elems(&List(vec![Number(*num)]), b),
        (List(_), Number(num)) => compare_elems(a, &List(vec![Number(*num)])),
        (List(vec_a), List(vec_b)) => {
            for i in 0..vec_a.len() {
                if i == vec_b.len() {
                    return Ordering::Greater;
                }
                let cmp_res = compare_elems(&vec_a[i], &vec_b[i]);
                if cmp_res == Ordering::Equal {
                    continue;
                } else {
                    return cmp_res;
                }
            }
            vec_a.len().cmp(&vec_b.len())
        }
    }
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        use Elem::*;
        match (self, other) {
            (Number(this), Number(that)) => this == that,
            (List(this), List(that)) => this.eq(that),
            _ => false,
        }
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_elems(self, other))
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Debug for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Elem::*;
        fn to_string(elem: &Elem) -> String {
            match elem {
                Number(num) => format!("{}", num),
                List(list) => {
                    let mut result = list.iter().map(to_string).collect::<Vec<_>>().join(",");
                    result.insert(0, '[');
                    result.push(']');
                    result
                }
            }
        }
        write!(f, "{}", to_string(self))
    }
}

#[derive(Debug)]
pub struct PacketPair {
    pub first: Elem,
    pub second: Elem,
}

impl From<Pair<'_, Rule>> for PacketPair {
    fn from(packet_pair: Pair<'_, Rule>) -> Self {
        fn from_number(line: Pair<'_, Rule>) -> Elem {
            Elem::Number(str::parse(line.as_str()).unwrap())
        }

        fn from_list(line: Pair<'_, Rule>) -> Elem {
            let list_items = line
                .into_inner()
                .map(|item| match item.as_rule() {
                    Rule::number => from_number(item),
                    Rule::list => from_list(item),
                    _ => unreachable!(),
                })
                .collect();
            Elem::List(list_items)
        }

        let mut lines_iter = packet_pair.into_inner();
        // There are exactly two "list"s, guaranteed.
        PacketPair {
            first: from_list(lines_iter.next().unwrap()),
            second: from_list(lines_iter.next().unwrap()),
        }
    }
}

impl PacketPair {
    pub fn is_right_order(&self) -> bool {
        compare_elems(&self.first, &self.second) != Ordering::Greater
    }
}

pub fn parse_pairs(mut reader: impl Read) -> Result<Vec<PacketPair>, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let parse_result = PacketParser::parse(Rule::file, buffer.as_str())?;
    // There is exactly one "file", guaranteed.
    let file = parse_result.into_iter().next().unwrap();
    Ok(file
        .into_inner()
        .filter(|item| item.as_rule() == Rule::packet_pair)
        .map(|packet_pair_parsed| packet_pair_parsed.into())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "."]
    struct Asset;

    #[test]
    fn ensure_ivariants() {
        let asset = Asset::get("test_input.txt").unwrap();
        let pairs = parse_pairs(asset.data.as_ref()).unwrap();
        assert_eq!(pairs.len(), 8);

        let mut lines: Vec<Elem> = pairs
            .into_iter()
            .flat_map(|pair| vec![pair.first, pair.second])
            .collect();

        lines.sort_by(compare_elems);

        for i in 0..lines.len() {
            assert!(lines[i] == lines[i]);
            assert!(lines[i].cmp(&lines[i]) == Ordering::Equal);
            for j in 0..i {
                assert!(lines[j].cmp(&lines[i]) == Ordering::Less);
                assert!(lines[i].cmp(&lines[j]) == Ordering::Greater);
                assert!(lines[j].eq(&lines[i]) == false);
                assert!(lines[i].eq(&lines[j]) == false);
            }
            for j in i + 1..lines.len() {
                assert!(lines[j].cmp(&lines[i]) == Ordering::Greater);
                assert!(lines[i].cmp(&lines[j]) == Ordering::Less);
                assert!(lines[j].eq(&lines[i]) == false);
                assert!(lines[i].eq(&lines[j]) == false);
            }
        }
    }
}
