pub struct Solution {}

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let score = nums[0] + nums[1];
        for i in (2..nums.len() - 1).step_by(2) {
            if score != nums[i] + nums[i + 1] {
                break;
            }
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3038() {
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 4, 5]), 2);
        assert_eq!(
            Solution::max_operations(vec![1, 5, 3, 3, 4, 1, 3, 2, 2, 3]),
            2
        );
        assert_eq!(Solution::max_operations(vec![5, 3]), 1);
    }
}
