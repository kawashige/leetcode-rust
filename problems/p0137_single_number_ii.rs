pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        for i in 0..nums.len() {
            if !((0 < i && nums[i - 1] == nums[i])
                || (i < nums.len() - 1 && nums[i] == nums[i + 1]))
            {
                return nums[i];
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0137() {
        assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2]));
        assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
    }
}
