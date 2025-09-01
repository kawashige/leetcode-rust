pub struct Solution {}

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut left = vec![0; nums.len()];
        let mut right = vec![0; nums.len()];
        for i in 0..nums.len() {
            left[i] += nums[i];
            if 0 < i {
                left[i] += left[i - 1];
            }
            right[nums.len() - 1 - i] += nums[nums.len() - 1 - i];
            if 0 < i {
                right[nums.len() - 1 - i] += right[nums.len() - i];
            }
        }

        let mut result = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                if right[i] == left[i] {
                    result += 2;
                } else if (right[i] - left[i]).abs() == 1 {
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
    fn test_3354() {
        assert_eq!(Solution::count_valid_selections(vec![1, 0, 2, 0, 3]), 2);
        assert_eq!(
            Solution::count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0]),
            0
        );
    }
}
