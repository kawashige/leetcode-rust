pub struct Solution {}

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        (1..2_usize.pow(nums.len() as u32))
            .map(|i| {
                (0..nums.len()).fold(
                    0,
                    |acc, j| if i & 1 << j != 0 { acc ^ nums[j] } else { acc },
                )
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1863() {
        assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
        assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
        assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
