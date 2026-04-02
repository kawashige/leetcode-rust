pub struct Solution {}

impl Solution {
    pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        let mut dp = vec![0; weight.len()];
        for i in 1..weight.len() {
            dp[i] = dp[i - 1];
            if weight[i] < weight[i - 1] {
                dp[i] = dp[i].max(if 1 < i { dp[i - 2] } else { 0 } + 1);
            }
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3638() {
        assert_eq!(Solution::max_balanced_shipments(vec![2, 5, 1, 4, 3]), 2);
        assert_eq!(Solution::max_balanced_shipments(vec![4, 4]), 0);
    }
}
