pub struct Solution {}

impl Solution {
    pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        (1..nums.len())
            .map(|i| nums[i] - nums[i - 1])
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2740() {
        assert_eq!(Solution::find_value_of_partition(vec![1, 3, 2, 4]), 1);
        assert_eq!(Solution::find_value_of_partition(vec![100, 1, 10]), 9);
    }
}
