pub struct Solution {}

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];
        for i in 0..arr.len() {
            dp[i + 1] = arr[i];
            let mut max = arr[i];
            for j in 0..(k as usize) {
                if j > i {
                    break;
                }
                max = max.max(arr[i - j]);
                dp[i + 1] = dp[i + 1].max(dp[i - j] + max * (j as i32 + 1));
            }
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1043() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
            84
        );
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
            83
        );
        assert_eq!(Solution::max_sum_after_partitioning(vec![1], 1), 1);
    }
}
