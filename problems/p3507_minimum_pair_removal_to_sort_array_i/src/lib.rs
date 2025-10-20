pub struct Solution {}

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut result = 0;

        while 1 < nums.len() {
            let mut is_sorted = nums[0] <= nums[1];
            let mut min_index = 0;
            let mut min_sum = nums[0] + nums[1];

            for i in 1..nums.len() - 1 {
                if nums[i] > nums[i + 1] {
                    is_sorted = false;
                }
                if nums[i] + nums[i + 1] < min_sum {
                    min_index = i;
                    min_sum = nums[i] + nums[i + 1];
                }
            }
            if is_sorted {
                break;
            }
            result += 1;
            nums = nums[..min_index]
                .iter()
                .cloned()
                .chain(std::iter::once(nums[min_index] + nums[min_index + 1]))
                .chain(nums[min_index + 2..].iter().cloned())
                .collect()
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3507() {
        assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
        assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
    }
}
