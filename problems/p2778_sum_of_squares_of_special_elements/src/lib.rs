pub struct Solution {}

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .map(|i| {
                if nums.len() % (i + 1) == 0 {
                    nums[i] * nums[i]
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2778() {
        assert_eq!(Solution::sum_of_squares(vec![1, 2, 3, 4]), 21);
        assert_eq!(Solution::sum_of_squares(vec![2, 7, 1, 19, 18, 3]), 63);
    }
}
