pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let mut queries = queries.into_iter().zip(0..).collect::<Vec<_>>();
        queries.sort_unstable();
        let mut result = vec![0; queries.len()];

        let total_sum: i64 = nums.iter().map(|n| *n as i64).sum();
        let mut nums = nums;
        nums.sort_unstable();
        let mut sum = 0;
        let mut j = 0;

        for i in 0..queries.len() {
            while j < nums.len() && nums[j] < queries[i].0 {
                sum += nums[j] as i64;
                j += 1;
            }
            result[queries[i].1] = (sum - queries[i].0 as i64 * j as i64).abs()
                + (total_sum - sum - queries[i].0 as i64 * (nums.len() - j) as i64).abs()
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2602() {
        assert_eq!(
            Solution::min_operations(vec![3, 1, 6, 8], vec![1, 5]),
            vec![14, 10]
        );
        assert_eq!(
            Solution::min_operations(vec![2, 9, 6, 3], vec![10]),
            vec![20]
        );
    }
}
