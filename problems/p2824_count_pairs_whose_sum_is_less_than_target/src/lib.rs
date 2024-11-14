pub struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] < target {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2824() {
        assert_eq!(Solution::count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(Solution::count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10);
    }
}
