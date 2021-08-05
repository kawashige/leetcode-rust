pub struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut x = 0;
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                if x == 2 {
                    return false;
                }
                x = 1
            } else if nums[i - 1] > nums[i] {
                if x == 1 {
                    return false;
                }
                x = 2
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0896() {
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
        assert!(Solution::is_monotonic(vec![1, 2, 4, 5]));
        assert!(Solution::is_monotonic(vec![1, 1, 1]));
    }
}
