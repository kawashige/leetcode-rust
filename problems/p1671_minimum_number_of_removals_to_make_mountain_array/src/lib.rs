pub struct Solution {}

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut left = vec![1; nums.len()];
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    left[i] = left[i].max(left[j] + 1);
                }
            }
        }
        let mut right = vec![1; nums.len()];
        for i in (0..nums.len()).rev() {
            for j in (i + 1..nums.len()).rev() {
                if nums[j] < nums[i] {
                    right[i] = right[i].max(right[j] + 1);
                }
            }
        }
        (1..nums.len() - 1)
            .map(|i| {
                if left[i] == 1 || right[i] == 1 {
                    std::usize::MAX
                } else {
                    nums.len() - (left[i] + right[i] - 1)
                }
            })
            .min()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1671() {
        assert_eq!(
            Solution::minimum_mountain_removals(vec![100, 92, 89, 77, 74, 66, 64, 66, 64]),
            6
        );
        assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
        assert_eq!(
            Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
            3
        );
    }
}
