use std::collections::HashMap;

pub fn detect_start_of_unique_window(
    input: &str,
    uniq_len: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let input_bytes: &[u8] = input.as_bytes();
    let mut window: HashMap<u8, usize> = HashMap::with_capacity(uniq_len);

    for (idx, &byte) in input_bytes.iter().enumerate() {
        *window.entry(byte).or_default() += 1;
        if idx >= uniq_len {
            let prev = input_bytes[idx - uniq_len];
            if let Some(&v) = window.get(&prev) {
                if v > 1 {
                    *window.entry(prev).or_default() -= 1;
                } else {
                    window.remove(&prev);
                }
            }
            if window.len() == uniq_len {
                return Ok(idx + 1);
            }
        }
    }
    Ok(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(
            detect_start_of_unique_window("mjqjpqmgbljsp", 4).unwrap(),
            7
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            detect_start_of_unique_window("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(),
            11
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            detect_start_of_unique_window("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(),
            5
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            detect_start_of_unique_window("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(),
            6
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            detect_start_of_unique_window("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(),
            10
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            detect_start_of_unique_window("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
            19
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            detect_start_of_unique_window("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(),
            23
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            detect_start_of_unique_window("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(),
            23
        );
    }

    #[test]
    fn test_8() {
        assert_eq!(
            detect_start_of_unique_window("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(),
            29
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(
            detect_start_of_unique_window("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(),
            26
        );
    }
}
