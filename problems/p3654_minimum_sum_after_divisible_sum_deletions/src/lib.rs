use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_array_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut dp = vec![0; nums.len() + 1];
        let mut map = HashMap::new();
        map.insert(0, 0);
        let k = k as i64;

        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
            let reminder = prefix_sum[i + 1] % k;
            dp[i + 1] = dp[i] + nums[i] as i64;
            if let Some(v) = map.get(&reminder) {
                dp[i + 1] = dp[i + 1].min(*v);
                if &dp[i + 1] < v {
                    map.insert(reminder, dp[i + 1]);
                }
            } else {
                map.insert(reminder, dp[i + 1]);
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3654() {
        assert_eq!(Solution::min_array_sum(vec![1, 1, 1], 2), 1);
        assert_eq!(Solution::min_array_sum(vec![3, 1, 4, 1, 5], 3), 5);
    }
}
