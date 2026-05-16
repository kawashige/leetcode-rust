pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }

        let mut result = 2;
        let mut l = 2;
        for i in 2..nums.len() {
            if nums[i - 2] + nums[i - 1] == nums[i] {
                l += 1;
            } else {
                result = result.max(l);
                l = 2;
            }
        }

        result.max(l)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3708() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1, 1, 2, 3, 5, 1]), 5);
        assert_eq!(Solution::longest_subarray(vec![5, 2, 7, 9, 16]), 5);
        assert_eq!(
            Solution::longest_subarray(vec![1000000000, 1000000000, 1000000000]),
            2
        );
    }
}

fn main() {}
