pub struct Solution {}

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![1; pairs.len()];
        pairs.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        for i in 1..pairs.len() {
            let mut max = 1;
            for j in 0..i {
                if pairs[j][1] < pairs[i][0] {
                    max = std::cmp::max(max, dp[j] + 1);
                }
            }
            dp[i] = max;
        }
        dp[pairs.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0645() {
        assert_eq!(
            Solution::find_longest_chain(vec![
                vec![-6, 9],
                vec![1, 6],
                vec![8, 10],
                vec![-1, 4],
                vec![-6, -2],
                vec![-9, 8],
                vec![-5, 3],
                vec![0, 3]
            ]),
            3
        );
        assert_eq!(
            Solution::find_longest_chain(vec![
                vec![9, 10],
                vec![-4, 9],
                vec![-5, 6],
                vec![-5, 9],
                vec![8, 9]
            ]),
            2
        );
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        );
    }
}
