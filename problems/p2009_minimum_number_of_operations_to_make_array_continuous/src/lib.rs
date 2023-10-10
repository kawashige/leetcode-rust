pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut duplicate = vec![0; nums.len() + 1];
        for i in 1..nums.len() {
            duplicate[i + 1] = duplicate[i];
            if nums[i - 1] == nums[i] {
                duplicate[i + 1] += 1;
            }
        }

        let mut j = 0;
        let mut min = std::usize::MAX;

        for i in 0..nums.len() {
            while j + 1 < nums.len() && nums[j + 1] <= nums[i] + nums.len() as i32 - 1 {
                j += 1;
            }
            min = min.min(i + nums.len() - 1 - j + duplicate[j + 1] - duplicate[i]);
        }

        min as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2009() {
        assert_eq!(Solution::min_operations(vec![8, 5, 9, 9, 8, 4]), 2);
        assert_eq!(Solution::min_operations(vec![4, 2, 5, 3]), 0);
        assert_eq!(Solution::min_operations(vec![1, 2, 3, 5, 6]), 1);
        assert_eq!(Solution::min_operations(vec![1, 10, 100, 1000]), 3);
    }
}
