pub struct Solution {}

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut sum = vec![0; nums.len()];
        sum[0] = nums[0];
        for i in 1..nums.len() {
            sum[i] = sum[i - 1] + nums[i];
        }
        queries
            .into_iter()
            .map(|q| match sum.binary_search(&(q + 1)) {
                Ok(i) => i as i32,
                Err(i) => i as i32,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2389() {
        assert_eq!(
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            vec![2, 3, 4]
        );
        assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }
}
