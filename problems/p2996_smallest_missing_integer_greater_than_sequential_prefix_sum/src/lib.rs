pub struct Solution {}

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                break;
            }
            sum += nums[i];
        }
        while nums.contains(&sum) {
            sum += 1;
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2996() {
        assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
        assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    }
}
