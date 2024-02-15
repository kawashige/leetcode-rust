pub struct Solution {}

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut sum = 0;
        let mut result = -1;

        for i in 0..nums.len() {
            if 1 < i && (nums[i] as i64) < sum {
                result = sum + nums[i] as i64;
            }
            sum += nums[i] as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2971() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 5]), 15);
        assert_eq!(Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 50]), -1);
    }
}
