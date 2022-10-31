pub struct Solution {}

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut min = 0;
        let mut sum = 0;
        let mut result = 0;

        for num in nums {
            sum += num;
            result = result.max((sum - min).abs().max((sum - max).abs()));
            max = max.max(sum);
            min = min.min(sum);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1749() {
        assert_eq!(Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
        assert_eq!(Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
    }
}
