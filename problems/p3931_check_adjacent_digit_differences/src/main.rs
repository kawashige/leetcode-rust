pub struct Solution {}

impl Solution {
    pub fn is_adjacent_diff_at_most_two(s: String) -> bool {
        (1..s.len()).all(|i| {
            ((s.as_bytes()[i] - b'0') as i32 - (s.as_bytes()[i - 1] - b'0') as i32).abs() < 3
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3931() {
        assert!(Solution::is_adjacent_diff_at_most_two("132".to_string()));
        assert!(!Solution::is_adjacent_diff_at_most_two("129".to_string()));
    }
}

fn main() {}
