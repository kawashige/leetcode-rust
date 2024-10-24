pub struct Solution {}

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        (0..nums.len() - 1).all(|i| nums[i] == i as i32 + 1)
            && nums.last() == Some(&(nums.len() as i32 - 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2784() {
        assert!(!Solution::is_good(vec![2, 1, 3]));
        assert!(Solution::is_good(vec![1, 3, 3, 2]));
        assert!(Solution::is_good(vec![1, 1]));
        assert!(!Solution::is_good(vec![3, 4, 4, 1, 2, 1]));
    }
}
