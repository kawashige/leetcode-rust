use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut seen = HashSet::new();
        seen.insert((0, 0));

        let mut x = 0;
        let mut y = 0;

        for b in path.as_bytes() {
            match b {
                b'N' => y += 1,
                b'S' => y -= 1,
                b'E' => x += 1,
                b'W' => x -= 1,
                _ => unreachable!(),
            };
            if !seen.insert((x, y)) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1496() {
        assert!(!Solution::is_path_crossing("NES".to_string()));
        assert!(Solution::is_path_crossing("NESWW".to_string()));
    }
}
