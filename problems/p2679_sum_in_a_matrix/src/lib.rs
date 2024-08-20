pub struct Solution {}

impl Solution {
    pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        for i in 0..nums.len() {
            nums[i].sort_unstable();
        }

        let mut result = 0;
        for j in 0..nums[0].len() {
            let mut score = 0;
            for i in 0..nums.len() {
                score = score.max(nums[i][j]);
            }
            result += score;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2670() {
        assert_eq!(
            Solution::matrix_sum(vec![
                vec![7, 2, 1],
                vec![6, 4, 2],
                vec![6, 5, 3],
                vec![3, 2, 1]
            ]),
            15
        );
        assert_eq!(Solution::matrix_sum(vec![vec![1]]), 1);
    }
}
