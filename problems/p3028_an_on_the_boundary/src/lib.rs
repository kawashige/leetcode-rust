pub struct Solution {}

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut p = 0;

        for i in 0..nums.len() {
            p += nums[i];
            if p == 0 {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3028() {
        assert_eq!(Solution::return_to_boundary_count(vec![2, 3, -5]), 1);
        assert_eq!(Solution::return_to_boundary_count(vec![3, 2, -3, -4]), 0);
    }
}
