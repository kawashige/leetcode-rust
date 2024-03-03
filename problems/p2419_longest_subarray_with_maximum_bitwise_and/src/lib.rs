pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut j = 0;
        let mut max = nums[0];
        let mut result = 1;

        for i in 1..nums.len() {
            while j < i && nums[j] != nums[i] {
                j += 1;
            }
            if max < nums[i] {
                result = i - j + 1;
                max = nums[i];
            } else if max == nums[i] {
                result = result.max(i - j + 1);
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2419() {
        assert_eq!(
            Solution::longest_subarray(vec![
                96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979
            ]),
            1
        );
        assert_eq!(Solution::longest_subarray(vec![10, 2, 2, 2, 3, 3, 5, 5]), 1);
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 4]), 1);
    }
}
