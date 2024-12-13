pub struct Solution {}

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        (0..nums.len())
            .filter(|num| num.count_ones() as i32 == k)
            .map(|i| nums[i])
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2859() {
        assert_eq!(
            Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1),
            13
        );
        assert_eq!(
            Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2),
            1
        );
    }
}
