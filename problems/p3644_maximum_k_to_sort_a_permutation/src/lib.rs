pub struct Solution {}

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        let must_swap = (0..nums.len())
            .filter(|i| nums[*i] != sorted[*i])
            .collect::<Vec<_>>();
        if must_swap.is_empty() {
            0
        } else {
            must_swap
                .into_iter()
                .fold(std::i32::MAX, |acc, i| acc & nums[i])
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3644() {
        assert_eq!(Solution::sort_permutation(vec![0, 3, 2, 1]), 1);
        assert_eq!(Solution::sort_permutation(vec![0, 1, 3, 2]), 2);
        assert_eq!(Solution::sort_permutation(vec![3, 2, 1, 0]), 0);
    }
}
