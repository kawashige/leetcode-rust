pub struct Solution {}

impl Solution {
    pub fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
        for i in (0..nums.len() - 1).rev() {
            if nums[i + 1] <= nums[i] {
                return i as i32 + 1;
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3818() {
        assert_eq!(
            Solution::minimum_prefix_length(vec![1, -1, 2, 3, 3, 4, 5]),
            4
        );
        assert_eq!(Solution::minimum_prefix_length(vec![4, 3, -2, -5]), 3);
        assert_eq!(Solution::minimum_prefix_length(vec![1, 2, 3, 4]), 0);
    }
}

fn main() {}
