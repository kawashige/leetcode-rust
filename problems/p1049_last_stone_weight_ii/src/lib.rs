pub struct Solution {}

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let mut dp = vec![vec![false; 100 * stones.len() * 2 + 1]; stones.len() + 1];
        dp[0][100 * stones.len()] = true;

        for i in 0..stones.len() {
            for j in 0..dp[i].len() {
                if dp[i][j] {
                    dp[i + 1][j - stones[i] as usize] = true;
                    dp[i + 1][j + stones[i] as usize] = true;
                }
            }
        }

        ((100 * stones.len()..dp[0].len())
            .find(|x| dp.last().unwrap()[*x])
            .unwrap()
            - 100 * stones.len()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1049() {
        assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
        assert_eq!(Solution::last_stone_weight_ii(vec![1, 2]), 1);
    }
}
