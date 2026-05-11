pub struct Solution {}

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut num = 0;
        let mut all_zero = true;

        for i in 0..nums.len() {
            num ^= nums[i];
            if nums[i] != 0 {
                all_zero = false;
            }
        }

        if num != 0 {
            nums.len() as i32
        } else if all_zero {
            0
        } else {
            nums.len() as i32 - 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3702() {
        assert_eq!(
            Solution::longest_subsequence(vec![0, 0, 7, 0, 0, 0, 7, 0, 0]),
            8
        );
        assert_eq!(Solution::longest_subsequence(vec![7, 0, 7, 0, 0]), 4);
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2);
        assert_eq!(Solution::longest_subsequence(vec![2, 3, 4]), 3);
    }
}

fn main() {}
