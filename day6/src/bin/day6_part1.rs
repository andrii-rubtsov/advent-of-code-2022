/*! See https://adventofcode.com/2022/day/6 */

use std::collections::HashMap;

fn detect_start_of_unique_window(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let input_vec: Vec<char> = input.chars().collect();
    let input_chars: &[char] = &input_vec[..];

    let mut window: HashMap<char, usize> = HashMap::with_capacity(4);

    for (idx, &char) in input_chars.iter().enumerate() {
        *window.entry(char).or_default() += 1;
        if idx >= 4 {
            let prev = input_chars[idx - 4];
            if let Some(&v) = window.get(&prev) {
                if v > 1 {
                    *window.entry(prev).or_default() -= 1;
                } else {
                    window.remove(&prev);
                }
            }
            if window.len() == 4 {
                return Ok(idx + 1);
            }
        }
    }
    Ok(usize::MAX)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_string = std::fs::read_to_string(utils::find_empirically("day6/input.txt"))?;
    let number = detect_start_of_unique_window(&input_string)?;
    println!("Number of characters before comm start: {number}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(detect_start_of_unique_window("mjqjpqmgbljsp").unwrap(), 7);
    }

    #[test]
    fn test_1() {
        assert_eq!(
            detect_start_of_unique_window("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
            11
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            detect_start_of_unique_window("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(),
            5
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            detect_start_of_unique_window("nppdvjthqldpwncqszvftbrmjlhg").unwrap(),
            6
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            detect_start_of_unique_window("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
            10
        );
    }
}
