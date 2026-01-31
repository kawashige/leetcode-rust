pub struct Solution {}

impl Solution {
    pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        let mut size = 1;
        let mut cur = nums[0];

        for i in 1..nums.len() {
            if cur <= nums[i] {
                size += 1;
                cur = nums[i];
            }
        }

        size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3523() {
        assert_eq!(Solution::maximum_possible_size(vec![4, 2, 5, 3, 5]), 3);
        assert_eq!(Solution::maximum_possible_size(vec![1, 2, 3]), 3);
    }
}
