pub struct Solution {}

impl Solution {
    const M: i32 = 1_000_000_007;

    pub fn recurse(
        bit: usize,
        v: usize,
        dp: &mut Vec<Vec<i32>>,
        connected: &Vec<Vec<bool>>,
    ) -> i32 {
        if dp[bit][v] != -1 {
            return dp[bit][v];
        }
        if bit == 1 << v {
            return 1;
        }
        let mut result = 0;

        let prev_bit = bit & !(1 << v);

        for u in 0..dp[0].len() {
            if !connected[v][u] || prev_bit & (1 << u) == 0 {
                continue;
            }
            result += Self::recurse(prev_bit, u, dp, connected);
            result %= Self::M
        }

        dp[bit][v] = result;
        dp[bit][v]
    }

    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![-1; nums.len()]; 2_usize.pow(nums.len() as u32)];
        for i in 0..nums.len() {
            dp[0 | 1 << i][i] = 1;
        }
        let mut connected = vec![vec![false; nums.len()]; nums.len()];
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] % nums[j] == 0 || nums[j] % nums[i] == 0 {
                    connected[i][j] = true;
                    connected[j][i] = true;
                }
            }
        }

        let mut result = 0;
        for v in 0..nums.len() {
            result += Self::recurse(dp.len() - 1, v, &mut dp, &connected);
            result %= Self::M;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2741() {
        assert_eq!(Solution::special_perm(vec![2, 3, 6]), 2);
        assert_eq!(Solution::special_perm(vec![1, 4, 3]), 2);
    }
}
