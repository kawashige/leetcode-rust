pub struct Solution {}

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![false; n as usize + 1];
        for i in 1..=n {
            for s in (1..=i).map(|v| (v as i32).pow(2)).take_while(|v| v <= &i) {
                if !dp[(i - s) as usize] {
                    dp[i as usize] = true;
                    break;
                }
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day25() {
        assert!(Solution::winner_square_game(1));
        assert!(!Solution::winner_square_game(2));
        assert!(Solution::winner_square_game(4));
        assert!(!Solution::winner_square_game(7));
        assert!(!Solution::winner_square_game(17));
    }
}
