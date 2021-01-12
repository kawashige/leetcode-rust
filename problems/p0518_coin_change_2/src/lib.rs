pub struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;
        for i in 0..coins.len() {
            let v = coins[i] as usize;
            if amount < v {
                continue;
            }
            for j in 0..=(amount - v) {
                if 0 < dp[j] {
                    dp[j + v] += dp[j];
                }
            }
        }
        dp[amount]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0518() {
        assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
        assert_eq!(0, Solution::change(3, vec![2]));
        assert_eq!(1, Solution::change(10, vec![10]));
    }
}
