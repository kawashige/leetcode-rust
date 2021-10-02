pub struct Solution {}

impl Solution {
    pub fn is_ok(hp: i32, dungeon: &Vec<Vec<i32>>) -> bool {
        let mut dp = vec![vec![std::i32::MIN; dungeon[0].len()]; dungeon.len()];
        dp[0][0] = hp + dungeon[0][0];

        for i in 0..dungeon.len() {
            for j in 0..dungeon[0].len() {
                if i == 0 && j == 0 {
                    continue;
                }

                if i > 0 && dp[i - 1][j] > 0 && dp[i - 1][j] + dungeon[i][j] > 0 {
                    dp[i][j] = dp[i - 1][j] + dungeon[i][j];
                }
                if j > 0 && dp[i][j - 1] > 0 && dp[i][j - 1] + dungeon[i][j] > 0 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1] + dungeon[i][j]);
                }
            }
        }

        dp[dungeon.len() - 1][dungeon[0].len() - 1] > 0
    }

    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        if Self::is_ok(1, &dungeon) {
            return 1;
        }

        let mut ng = 1;
        let mut ok = ((dungeon.len() + dungeon[0].len() - 1) * 1000 + 1) as i32;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, &dungeon) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0174() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5]
            ]),
            7
        );
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![0]]), 1);
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![-200]]), 201);
        assert_eq!(
            Solution::calculate_minimum_hp(vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]]),
            3
        );
    }
}
