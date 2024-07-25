pub struct Solution {}

impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let mut result = vec![0; nums.len()];
        result[0] = nums[0] as i64 * 2;
        let mut max = nums[0];

        for i in 1..nums.len() {
            max = nums[i].max(max);
            result[i] = result[i - 1] + nums[i] as i64 + max as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2640() {
        assert_eq!(
            Solution::find_prefix_score(vec![2, 3, 7, 5, 10]),
            vec![4, 10, 24, 36, 56]
        );
        assert_eq!(
            Solution::find_prefix_score(vec![1, 1, 2, 4, 8, 16]),
            vec![2, 4, 8, 16, 32, 64]
        );
    }
}
