pub struct Solution {}

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut dp = vec![vec![0; locations.len()]; fuel as usize + 1];
        dp[fuel as usize][start as usize] = 1;
        let mut result = if start == finish { 1 } else { 0 };

        for i in (1..=fuel as usize).rev() {
            for j in 0..locations.len() {
                for k in 0..locations.len() {
                    if j == k {
                        continue;
                    }
                    let d = (locations[j] - locations[k]).abs() as usize;
                    if d <= i {
                        dp[i - d][k] += dp[i][j];
                        dp[i - d][k] %= M;
                        if k == finish as usize {
                            result += dp[i][j];
                            result %= M;
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1575() {
        assert_eq!(Solution::count_routes(vec![2, 1, 5], 0, 0, 3), 2);
        assert_eq!(Solution::count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
        assert_eq!(Solution::count_routes(vec![4, 3, 1], 1, 0, 6), 5);
        assert_eq!(Solution::count_routes(vec![5, 2, 1], 0, 2, 3), 0);
    }
}
