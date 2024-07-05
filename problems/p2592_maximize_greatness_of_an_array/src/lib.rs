pub struct Solution {}

impl Solution {
    pub fn maximize_greatness(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut result = 0;
        let mut j = 0;
        for i in 0..nums.len() {
            while j < nums.len() && nums[j] <= nums[i] {
                j += 1;
            }
            if j == nums.len() {
                break;
            }
            result += 1;
            j += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2592() {
        assert_eq!(Solution::maximize_greatness(vec![1, 3, 5, 2, 1, 3, 1]), 4);
        assert_eq!(Solution::maximize_greatness(vec![1, 2, 3, 4]), 3);
    }
}
