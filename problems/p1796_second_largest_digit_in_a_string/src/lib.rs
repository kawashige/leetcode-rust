pub struct Solution {}

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut digits = s.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
        digits.sort_unstable();
        digits.dedup();
        if digits.len() < 2 {
            -1
        } else {
            digits[digits.len() - 2].to_digit(10).unwrap() as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1796() {
        assert_eq!(Solution::second_highest("dfa12321afd".to_string()), 2);
        assert_eq!(Solution::second_highest("abc1111".to_string()), -1);
    }
}
