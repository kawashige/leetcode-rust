pub struct Solution {}

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len() / 2;
        let mut dp = vec![vec![1000 * n as i32 + 1; n + 1]; n + 1];

        for i in 0..costs.len() {
            for a in (0..=n).rev() {
                for b in (0..=n).rev() {
                    if a < n {
                        dp[a + 1][b] = dp[a + 1][b].min(dp[a][b] + costs[i][0]);
                    }
                    if b < n {
                        dp[a][b + 1] = dp[a][b + 1].min(dp[a][b] + costs[i][1]);
                    }
                }
            }
            dp[1][0] = dp[1][0].min(costs[i][0]);
            dp[0][1] = dp[0][1].min(costs[i][1]);
        }

        dp[n][n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1029() {
        assert_eq!(
            Solution::two_city_sched_cost(vec![
                vec![10, 20],
                vec![30, 200],
                vec![400, 50],
                vec![30, 20]
            ]),
            110
        );
        assert_eq!(
            Solution::two_city_sched_cost(vec![
                vec![259, 770],
                vec![448, 54],
                vec![926, 667],
                vec![184, 139],
                vec![840, 118],
                vec![577, 469]
            ]),
            1859
        );
        assert_eq!(
            Solution::two_city_sched_cost(vec![
                vec![515, 563],
                vec![451, 713],
                vec![537, 709],
                vec![343, 819],
                vec![855, 779],
                vec![457, 60],
                vec![650, 359],
                vec![631, 42]
            ]),
            3086
        );
    }
}
