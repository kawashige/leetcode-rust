pub struct Solution {}

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut zeros = 0;

        for num in nums {
            if num == 0 {
                zeros += 1;
                count += zeros;
            } else {
                zeros = 0;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2348() {
        assert_eq!(
            Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4]),
            6
        );
        assert_eq!(Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]), 9);
        assert_eq!(Solution::zero_filled_subarray(vec![2, 10, 2019]), 0);
    }
}
