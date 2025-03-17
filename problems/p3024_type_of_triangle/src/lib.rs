pub struct Solution {}

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_unstable();
        if nums[0] + nums[1] <= nums[2] {
            "none".to_string()
        } else if nums[0] == nums[1] && nums[1] == nums[2] {
            "equilateral".to_string()
        } else if nums[0] != nums[1] && nums[1] != nums[2] && nums[0] != nums[2] {
            "scalene".to_string()
        } else {
            "isosceles".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3024() {
        assert_eq!(
            Solution::triangle_type(vec![3, 3, 3]),
            "equilateral".to_string()
        );
        assert_eq!(
            Solution::triangle_type(vec![3, 4, 5]),
            "scalene".to_string()
        );
    }
}
