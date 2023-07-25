/*! See https://adventofcode.com/2022/day/13 */

#![allow(non_upper_case_globals)]

use day13::{compare_elems, parse_pairs, Elem, Elem::*};
use rust_embed::RustEmbed;
use std::io::Read;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn calc_decoder_key(reader: impl Read) -> Result<usize, Box<dyn std::error::Error>> {
    let mut lines: Vec<Elem> = parse_pairs(reader)?
        .into_iter()
        .flat_map(|pair| vec![pair.first, pair.second])
        .collect();
    let first_divider = List(vec![List(vec![Number(2)])]);
    let second_divider = List(vec![List(vec![Number(6)])]);

    lines.push(first_divider.clone());
    lines.push(second_divider.clone());

    lines.sort_by(compare_elems);

    let first_divided_idx = lines.binary_search(&first_divider).unwrap();
    let second_divided_idx = lines.binary_search(&second_divider).unwrap();

    Ok((first_divided_idx + 1) * (second_divided_idx + 1))
}

fn main() {
    let asset = Asset::get("input.txt").unwrap();
    let decoder_key = calc_decoder_key(asset.data.as_ref()).unwrap();
    println!("Decoder key: {decoder_key}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let decoder_key = calc_decoder_key(asset.data.as_ref()).unwrap();
        assert_eq!(decoder_key, 140);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let decoder_key = calc_decoder_key(asset.data.as_ref()).unwrap();
        assert_eq!(decoder_key, 22134);
    }
}
