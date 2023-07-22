/*! See https://adventofcode.com/2022/day/14 */

use day14::{Cave, Cell, Pos};
use rust_embed::RustEmbed;
use std::io::Read;

#[derive(RustEmbed)]
#[folder = "."]
struct Asset;

fn process_falling_sand_endless_floor(cave: &mut Cave, sand_pos: &Pos) -> bool {
    use Cell::*;

    if *cave.get(sand_pos) != Empty {
        return false;
    }

    if sand_pos.y == cave.max_height + 1 {
        cave.put_sand(sand_pos);
        return true;
    }
    let pos_down = Pos::new(sand_pos.x, sand_pos.y + 1);
    let pos_diag_left = Pos::new(sand_pos.x - 1, sand_pos.y + 1);
    let pos_diag_right = Pos::new(sand_pos.x + 1, sand_pos.y + 1);

    if *cave.get(&pos_down) == Empty {
        process_falling_sand_endless_floor(cave, &pos_down)
    } else if *cave.get(&pos_diag_left) == Empty {
        process_falling_sand_endless_floor(cave, &pos_diag_left)
    } else if *cave.get(&pos_diag_right) == Empty {
        process_falling_sand_endless_floor(cave, &pos_diag_right)
    } else {
        cave.put_sand(sand_pos);
        true
    }
}

fn total_came_to_rest_endless_floor(reader: impl Read) -> usize {
    let mut cave = Cave::parse(reader);
    let starting_point = Pos::new(500, 0);
    while process_falling_sand_endless_floor(&mut cave, &starting_point) {}
    cave.count_sand_cells()
}

fn main() {
    let asset = Asset::get("input.txt").unwrap();
    let sand_units_came_to_rest = total_came_to_rest_endless_floor(asset.data.as_ref());
    println!("Total sand units came to rest: {sand_units_came_to_rest}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let asset = Asset::get("test_input.txt").unwrap();
        let sand_units_came_to_rest = total_came_to_rest_endless_floor(asset.data.as_ref());
        assert_eq!(sand_units_came_to_rest, 93);
    }

    #[test]
    fn actual_input() {
        let asset = Asset::get("input.txt").unwrap();
        let sand_units_came_to_rest = total_came_to_rest_endless_floor(asset.data.as_ref());
        assert_eq!(sand_units_came_to_rest, 24377);
    }
}
