pub struct Solution {}

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left_positive = Some(-1);
        let mut left_negative = None;
        let mut sign = 1;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                sign = 1;
                left_positive = Some(i as i32);
                left_negative = None;
            } else {
                sign *= nums[i].signum();
                if sign == 1 {
                    if let Some(j) = left_positive {
                        max = max.max(i as i32 - j);
                    } else {
                        left_positive = Some(i as i32);
                    }
                } else {
                    if let Some(j) = left_negative {
                        max = max.max(i as i32 - j);
                    } else {
                        left_negative = Some(i as i32);
                    }
                }
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1567() {
        assert_eq!(Solution::get_max_len(vec![-1, 2]), 1);
        assert_eq!(Solution::get_max_len(vec![1, -2, -3, 4]), 4);
        assert_eq!(Solution::get_max_len(vec![1, -2, -3, 4]), 4);
        assert_eq!(Solution::get_max_len(vec![0, 1, -2, -3, -4]), 3);
        assert_eq!(Solution::get_max_len(vec![-1, -2, -3, 0, 1]), 2);
    }
}
