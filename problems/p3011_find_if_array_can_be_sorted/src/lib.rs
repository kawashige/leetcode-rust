pub struct Solution {}

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut nums = nums;

        for i in 0..nums.len() {
            for j in (i..nums.len() - 1).rev() {
                if nums[j + 1] < nums[j] {
                    if nums[j + 1].count_ones() != nums[j].count_ones() {
                        return false;
                    }
                    nums.swap(j + 1, j);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3011() {
        assert!(!Solution::can_sort_array(vec![18, 3, 8]));
        assert!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]));
        assert!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]));
        assert!(!Solution::can_sort_array(vec![3, 16, 8, 4, 2]));
    }
}
