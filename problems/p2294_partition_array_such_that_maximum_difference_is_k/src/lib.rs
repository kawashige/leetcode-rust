pub struct Solution {}

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut min = nums[0];
        let mut count = 1;

        for i in 1..nums.len() {
            if k < nums[i] - min {
                count += 1;
                min = nums[i];
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2294() {
        assert_eq!(Solution::partition_array(vec![3, 6, 1, 2, 5], 2), 2);
        assert_eq!(Solution::partition_array(vec![1, 2, 3], 1), 2);
        assert_eq!(Solution::partition_array(vec![2, 2, 4, 5], 0), 3);
    }
}
