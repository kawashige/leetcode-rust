pub struct Solution {}

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .step_by(2)
            .flat_map(|i| std::iter::repeat(nums[i + 1]).take(nums[i] as usize))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1313() {
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
            vec![2, 4, 4, 4]
        );
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 1, 2, 3]),
            vec![1, 3, 3]
        );
    }
}
