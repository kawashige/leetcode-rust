pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut proper_divisors = vec![std::i32::MAX; *nums.iter().max().unwrap() as usize + 1];
        for i in 2..proper_divisors.len() {
            if proper_divisors[i] != std::i32::MAX {
                continue;
            }
            for j in (i + i..proper_divisors.len()).step_by(i) {
                proper_divisors[j] = proper_divisors[j].min(i as i32);
            }
        }

        let mut nums = nums;
        let mut count = 0;
        for i in (0..nums.len() - 1).rev() {
            while nums[i + 1] < nums[i] {
                if proper_divisors[nums[i] as usize] == std::i32::MAX {
                    return -1;
                }
                nums[i] = proper_divisors[nums[i] as usize];
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3326() {
        assert_eq!(
            Solution::min_operations(vec![675, 47, 67, 89, 107, 127, 181, 193, 199]),
            -1
        );
        assert_eq!(Solution::min_operations(vec![25, 7]), 1);
        assert_eq!(Solution::min_operations(vec![7, 7, 6]), -1);
        assert_eq!(Solution::min_operations(vec![1, 1, 1, 1]), 0);
    }
}
