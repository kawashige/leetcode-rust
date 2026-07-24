pub struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut prefix_sum = vec![0; nums.len()];
        let mut suffix_min = vec![std::i64::MAX; nums.len()];
        prefix_sum[0] = nums[0] as i64;
        suffix_min[n - 1] = nums[n - 1] as i64;
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i] as i64;
        }
        for i in (0..n - 1).rev() {
            suffix_min[i] = suffix_min[i + 1].min(nums[i] as i64);
        }

        (0..n - 1)
            .map(|i| prefix_sum[i] - suffix_min[i + 1])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3788() {
        assert_eq!(Solution::maximum_score(vec![10, -1, 3, -4, -5]), 17);
        assert_eq!(Solution::maximum_score(vec![-7, -5, 3]), -2);
        assert_eq!(Solution::maximum_score(vec![1, 1]), 0);
    }
}

fn main() {}
