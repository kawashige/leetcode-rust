pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut result = std::i32::MAX;
        for i in m..=n {
            if result == 0 {
                break;
            }
            result &= i;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0201() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
        assert_eq!(0, Solution::range_bitwise_and(0, 1));
        assert_eq!(0, Solution::range_bitwise_and(0, 0));
        assert_eq!(0, Solution::range_bitwise_and(1, std::i32::MAX));
    }
}
