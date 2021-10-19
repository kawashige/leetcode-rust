pub struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        for i in (2..nums.len()).rev() {
            if nums[i] < nums[i - 1] + nums[i - 2] {
                return nums[i] + nums[i - 1] + nums[i - 2];
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0976() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
        assert_eq!(Solution::largest_perimeter(vec![1, 2, 1]), 0);
        assert_eq!(Solution::largest_perimeter(vec![3, 2, 3, 4]), 10);
        assert_eq!(Solution::largest_perimeter(vec![3, 6, 2, 3]), 8);
    }
}
