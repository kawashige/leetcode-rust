pub struct Solution {}

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut sum = std::i32::MAX;

        for k in 0..nums.len() {
            for j in 0..k {
                for i in 0..j {
                    if nums[i] < nums[j] && nums[k] < nums[j] {
                        sum = sum.min(nums[i] + nums[j] + nums[k]);
                    }
                }
            }
        }

        if sum == std::i32::MAX {
            -1
        } else {
            sum
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2908() {
        assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
        assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
