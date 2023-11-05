pub struct Solution {}

impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        while 1 < nums.len() {
            let mut new_nums = vec![0; nums.len() - 1];
            for i in 0..nums.len() - 1 {
                new_nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
            nums = new_nums;
        }
        nums[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2221() {
        assert_eq!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]), 8);
        assert_eq!(Solution::triangular_sum(vec![5]), 5);
    }
}
