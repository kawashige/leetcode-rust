pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut result = (nums.len() as i32).min(2);

        let mut left = vec![0; nums.len()];
        left[0] = 1;
        for i in 1..nums.len() {
            if nums[i - 1] <= nums[i] {
                left[i] = left[i - 1] + 1;
            } else {
                left[i] = 1;
            }

            result = result.max(left[i]);
            if i < nums.len() - 1 {
                result = result.max(left[i] + 1);
            }
        }

        let mut right = vec![0; nums.len()];
        right[nums.len() - 1] = 1;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] <= nums[i + 1] {
                right[i] = right[i + 1] + 1;
            } else {
                right[i] = 1;
            }

            result = result.max(right[i]);
            if 0 < i {
                result = result.max(right[i] + 1);
            }
        }

        for i in 1..nums.len() - 1 {
            if nums[i - 1] <= nums[i + 1] {
                result = result.max(left[i - 1] + right[i + 1] + 1);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3738() {
        assert_eq!(Solution::longest_subarray(vec![1, 5, -10, 5]), 4);
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 1, 2]), 4);
        assert_eq!(Solution::longest_subarray(vec![2, 2, 2, 2, 2]), 5);
    }
}

fn main() {}
