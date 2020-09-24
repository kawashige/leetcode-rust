pub struct Solution {}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if (i == 0 || nums[i - 1] < nums[i]) && (i == nums.len() - 1 || nums[i + 1] < nums[i]) {
                return i as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0162() {
        assert_eq!(0, Solution::find_peak_element(vec![3]));
        assert_eq!(2, Solution::find_peak_element(vec![1, 2, 3, 1]));
        assert_eq!(1, Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]));
    }
}
