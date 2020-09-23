pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let i = (1..nums.len())
            .find(|i| nums[*i] < nums[(nums.len() + i - 1) % nums.len()])
            .unwrap_or(0);
        nums[i]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0153() {
        assert_eq!(1, Solution::find_min(vec![3, 4, 5, 1, 2]));
        assert_eq!(1, Solution::find_min(vec![1]));
        assert_eq!(0, Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
    }
}
