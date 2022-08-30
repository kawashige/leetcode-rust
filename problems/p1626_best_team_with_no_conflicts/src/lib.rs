pub struct Solution {}

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut prayers = ages.into_iter().zip(scores.into_iter()).collect::<Vec<_>>();
        prayers.sort_unstable();

        let mut dp = vec![0; prayers.len()];
        dp[0] = prayers[0].1;

        for i in 1..prayers.len() {
            dp[i] = prayers[i].1;
            for j in 0..i {
                if prayers[j].1 <= prayers[i].1 {
                    dp[i] = dp[i].max(prayers[i].1 + dp[j]);
                }
            }
        }

        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1626() {
        assert_eq!(
            Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]),
            34
        );
        assert_eq!(
            Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1]),
            16
        );
        assert_eq!(
            Solution::best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1]),
            6
        );
    }
}
