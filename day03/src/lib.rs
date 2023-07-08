pub fn get_priority(c: char) -> usize {
    match c {
        'a'..='z' => (c as usize) - ('a' as usize) + 1,
        'A'..='Z' => (c as usize) - ('A' as usize) + 27,
        _ => panic!("Unable to determine priority for unsupported char '{}'", c),
    }
}
