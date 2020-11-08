pub struct Solution {}

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;

        for i in 2..=(n as usize) {
            let mut max = 0;
            for j in 1..i {
                max = std::cmp::max(max, dp[j] * (i - j));
                max = std::cmp::max(max, j * (i - j));
            }
            dp[i] = max;
        }
        dp[n as usize] as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0343() {
        assert_eq!(1, Solution::integer_break(2));
        assert_eq!(36, Solution::integer_break(10));
        assert_eq!(1549681956, Solution::integer_break(58));
    }
}
