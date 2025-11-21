pub struct Solution {}

impl Solution {
    pub fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
        let mut is_target = vec![false; source.len()];
        for i in &target_indices {
            is_target[*i as usize] = true;
        }
        let mut dp = vec![vec![target_indices.len() + 1; pattern.len()]; source.len() + 1];

        for i in 0..source.len() {
            for j in 0..pattern.len() {
                dp[i + 1][j] = dp[i][j];
                if source.as_bytes()[i] == pattern.as_bytes()[j] {
                    dp[i + 1][j] = dp[i + 1][j].min(
                        if j == 0 { 0 } else { dp[i][j - 1] } + if is_target[i] { 1 } else { 0 },
                    );
                }
            }
        }

        let count = *dp.last().unwrap().last().unwrap();
        if target_indices.len() <= count {
            0
        } else {
            (target_indices.len() - count) as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3316() {
        assert_eq!(
            Solution::max_removals("abbaa".to_string(), "aba".to_string(), vec![0, 1, 2]),
            1
        );
        assert_eq!(
            Solution::max_removals("bcda".to_string(), "d".to_string(), vec![0, 3]),
            2
        );
        assert_eq!(
            Solution::max_removals("dda".to_string(), "dda".to_string(), vec![0, 1, 2]),
            0
        );
        assert_eq!(
            Solution::max_removals(
                "yeyeykyded".to_string(),
                "yeyyd".to_string(),
                vec![0, 2, 3, 4]
            ),
            2
        );
    }
}
