use std::collections::HashSet;
pub struct Solution {}

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut square_numbers = HashSet::new();
        for i in 0..=c {
            let s = i * i;
            if c < s {
                break;
            }
            square_numbers.insert(s);
            if c - s <= s && square_numbers.contains(&(c - s)) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0633() {
        assert!(Solution::judge_square_sum(5));
        assert!(!Solution::judge_square_sum(3));
        assert!(Solution::judge_square_sum(4));
        assert!(Solution::judge_square_sum(2));
        assert!(Solution::judge_square_sum(1));
    }
}
