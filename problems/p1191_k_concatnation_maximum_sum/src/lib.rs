pub struct Solution {}

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        const M: i64 = 1_000_000_007;

        let mut sum = 0;
        let mut min_sum = 0;
        let mut max = 0;
        let mut max_prefix_sum = 0;
        for i in 0..arr.len() {
            sum += arr[i] as i64;
            max_prefix_sum = max_prefix_sum.max(sum);
            min_sum = min_sum.min(sum);
            max = max.max(sum - min_sum);
        }

        max = max.max(sum * k as i64 % M);

        if k == 1 {
            return max as i32;
        }

        let mut max_suffix_sum = 0;
        let mut sum = 0;
        for i in (0..arr.len()).rev() {
            sum += arr[i] as i64;
            max_suffix_sum = max_suffix_sum.max(sum);
        }

        (max.max(max_prefix_sum + max_suffix_sum + (sum * (k as i64 - 2 % M)).max(0)) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1191() {
        assert_eq!(
            Solution::k_concatenation_max_sum(
                vec![-9, 13, 4, -16, -12, -16, 3, -7, 5, -16, 16, 8, -1, -13, 15, 3],
                6
            ),
            36
        );
        assert_eq!(Solution::k_concatenation_max_sum(vec![1, 2], 3), 9);
        assert_eq!(Solution::k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
        assert_eq!(Solution::k_concatenation_max_sum(vec![-1, -2], 7), 0);
    }
}
