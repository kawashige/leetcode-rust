pub struct Solution {}

impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if (0..nums.len() - 1)
                .all(|j| nums[(i + j) % nums.len()] < nums[(i + j + 1) % nums.len()])
            {
                return if i == 0 { 0 } else { (nums.len() - i) as i32 };
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2855() {
        assert_eq!(Solution::minimum_right_shifts(vec![3, 4, 5, 1, 2]), 2);
        assert_eq!(Solution::minimum_right_shifts(vec![1, 3, 5]), 0);
        assert_eq!(Solution::minimum_right_shifts(vec![2, 1, 4]), -1);
    }
}
