pub struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut result = 0;
        let mut prev = '-';
        let mut power = 1;
        for c in s.chars() {
            if c == prev {
                power += 1;
            } else {
                result = std::cmp::max(result, power);
                power = 1;
                prev = c;
            }
        }
        std::cmp::max(result, power)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day3() {
        assert_eq!(0, Solution::max_power("".to_string()));
        assert_eq!(1, Solution::max_power("a".to_string()));
        assert_eq!(2, Solution::max_power("leetcode".to_string()));
        assert_eq!(5, Solution::max_power("abbcccddddeeeeedcba".to_string()));
        assert_eq!(5, Solution::max_power("triplepillooooow".to_string()));
        assert_eq!(11, Solution::max_power("hooraaaaaaaaaaay".to_string()));
        assert_eq!(1, Solution::max_power("tourist".to_string()));
    }
}
