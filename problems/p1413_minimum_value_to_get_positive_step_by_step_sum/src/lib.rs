pub struct Solution {}

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let min = nums
            .iter()
            .scan(0, |s, x| {
                *s += x;
                Some(*s)
            })
            .min()
            .unwrap();
        if min < 0 {
            min.abs() + 1
        } else {
            1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1413() {
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    }
}
