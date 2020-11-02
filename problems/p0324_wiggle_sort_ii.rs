pub struct Solution {}

use std::collections::VecDeque;
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort_by_key(|n| -n);
        let mut tmp = VecDeque::new();
        let mut j = nums.len() / 2;
        for i in 0..nums.len() {
            if i % 2 == 0 {
                tmp.push_back(nums[i]);
                nums[i] = nums[j];
                j += 1;
            } else {
                tmp.push_back(nums[i]);
                nums[i] = tmp.pop_front().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0324() {
        let mut input = vec![1, 1, 2, 1, 2, 2, 1];
        Solution::wiggle_sort(&mut input);
        assert_eq!(vec![1, 2, 1, 2, 1, 2, 1], input);

        let mut input = vec![4, 5, 5, 6];
        Solution::wiggle_sort(&mut input);
        assert_eq!(vec![5, 6, 4, 5], input);

        let mut input = vec![1, 5, 1, 1, 6, 4];
        Solution::wiggle_sort(&mut input);
        assert_eq!(vec![1, 6, 1, 5, 1, 4], input);

        let mut input = vec![1, 3, 2, 2, 3, 1];
        Solution::wiggle_sort(&mut input);
        assert_eq!(vec![2, 3, 1, 3, 1, 2], input);

        let mut input = vec![1, 1, 2, 2];
        Solution::wiggle_sort(&mut input);
        assert_eq!(vec![1, 2, 1, 2], input);

        let mut input = vec![1, 2];
        Solution::wiggle_sort(&mut input);
        assert_eq!(vec![1, 2], input);

        let mut input = vec![1, 1, 1, 2, 2, 2];
        Solution::wiggle_sort(&mut input);
        assert_eq!(vec![1, 2, 1, 2, 1, 2], input);
    }
}
