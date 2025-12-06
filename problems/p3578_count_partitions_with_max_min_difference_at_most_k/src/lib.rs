use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mod_val = 1_000_000_007i64;
        let mut dp = vec![0i64; n + 1];
        let mut prefix = vec![0i64; n + 1];
        let mut cnt = BTreeMap::new();

        dp[0] = 1;
        prefix[0] = 1;
        let mut j = 0;
        for i in 0..n {
            *cnt.entry(nums[i]).or_insert(0) += 1;
            // adjust window
            while j <= i && *cnt.keys().last().unwrap() - *cnt.keys().next().unwrap() > k {
                *cnt.get_mut(&nums[j]).unwrap() -= 1;
                if cnt[&nums[j]] == 0 {
                    cnt.remove(&nums[j]);
                }
                j += 1;
            }

            if j > 0 {
                dp[i + 1] = (prefix[i] - prefix[j - 1] + mod_val) % mod_val;
            } else {
                dp[i + 1] = prefix[i] % mod_val;
            }
            prefix[i + 1] = (prefix[i] + dp[i + 1]) % mod_val;
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3578() {
        assert_eq!(Solution::count_partitions(vec![9, 4, 1, 3, 7], 4), 6);
        assert_eq!(Solution::count_partitions(vec![3, 3, 4], 0), 2);
    }
}
