use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn min_all_one_multiple(k: i32) -> i32 {
        let mut rem = 1;
        let mut seen = HashSet::new();
        let mut result = 1;

        while rem != 0 {
            if seen.contains(&rem) {
                return -1;
            }
            seen.insert(rem);
            rem = (rem * 10 + 1) % k;
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3790() {
        assert_eq!(Solution::min_all_one_multiple(3), 3);
        assert_eq!(Solution::min_all_one_multiple(7), 6);
        assert_eq!(Solution::min_all_one_multiple(2), -1);
    }
}

fn main() {}
