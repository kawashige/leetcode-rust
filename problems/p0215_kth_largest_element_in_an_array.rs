pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums[nums.len() - k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0215() {
        assert_eq!(5, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
        assert_eq!(
            4,
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );
        assert_eq!(1, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 6));
        assert_eq!(1, Solution::find_kth_largest(vec![1], 1));
    }
}
