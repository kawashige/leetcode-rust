pub struct Solution {}

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            for j in 0..=i {
                if (j == 0 || (1..j).all(|k| nums[k - 1] < nums[k]))
                    && (i == nums.len() - 1
                        || (i + 1..nums.len() - 1).all(|k| nums[k] < nums[k + 1]))
                    && (j == 0 || i == nums.len() - 1 || nums[j - 1] < nums[i + 1])
                {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2970() {
        assert_eq!(Solution::incremovable_subarray_count(vec![1, 2, 3, 4]), 10);
        assert_eq!(Solution::incremovable_subarray_count(vec![6, 5, 7, 8]), 7);
        assert_eq!(Solution::incremovable_subarray_count(vec![8, 7, 6, 6]), 3);
    }
}
