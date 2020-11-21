pub struct Solution {}

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in 2..=n {
            for j in (1..i).rev() {
                let mut min = std::usize::MAX;
                for k in (j + 1)..i {
                    let max = k + std::cmp::max(dp[j][k - 1], dp[k + 1][i]);
                    min = std::cmp::min(min, max);
                }
                dp[j][i] = if j + 1 == i { j } else { min };
            }
        }
        println!("{:?}", dp);
        dp[1][n] as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0375() {
        assert_eq!(4, Solution::get_money_amount(4));
        assert_eq!(10, Solution::get_money_amount(7));
        assert_eq!(12, Solution::get_money_amount(8));
        assert_eq!(16, Solution::get_money_amount(10));
        assert_eq!(630, Solution::get_money_amount(139));
        assert_eq!(21, Solution::get_money_amount(12));
        assert_eq!(0, Solution::get_money_amount(1));
        assert_eq!(1, Solution::get_money_amount(2));
        assert_eq!(64, Solution::get_money_amount(25));
    }
}
