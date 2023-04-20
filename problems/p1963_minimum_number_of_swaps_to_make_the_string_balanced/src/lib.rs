pub struct Solution {}

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut balance = 0;
        let mut swaps = 0;

        for i in 0..s.len() {
            balance += if s.as_bytes()[i] == b'[' { 1 } else { -1 };
            if s.as_bytes()[i] == b']' && balance < 0 {
                swaps += 1;
                balance = 1;
            }
        }

        swaps
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1963() {
        assert_eq!(Solution::min_swaps("][][".to_string()), 1);
        assert_eq!(Solution::min_swaps("]]][[[".to_string()), 2);
        assert_eq!(Solution::min_swaps("[]".to_string()), 0);
    }
}
