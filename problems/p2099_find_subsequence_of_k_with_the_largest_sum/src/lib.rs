pub struct Solution {}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().zip(0..).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| b.cmp(&a));
        let mut target = nums[..k as usize].to_vec();
        target.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        target.into_iter().map(|t| t.0).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2099() {
        assert_eq!(Solution::max_subsequence(vec![2, 1, 3, 3], 2), vec![3, 3]);
        assert_eq!(
            Solution::max_subsequence(vec![-1, -2, 3, 4], 3),
            vec![-1, 3, 4]
        );
        assert_eq!(Solution::max_subsequence(vec![3, 4, 3, 3], 2), vec![4, 3]);
    }
}
