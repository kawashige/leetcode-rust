pub struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        32 + num.count_ones().saturating_sub(1) as i32 - num.leading_zeros() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day12() {
        assert_eq!(Solution::number_of_steps(1), 1);
        assert_eq!(Solution::number_of_steps(0), 0);
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
    }
}
