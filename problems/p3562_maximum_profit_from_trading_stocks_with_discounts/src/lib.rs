pub struct Solution {}

impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        let n = n as usize;
        let budget = budget as usize;
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for e in hierarchy {
            g[(e[0] - 1) as usize].push((e[1] - 1) as usize);
        }

        fn dfs(
            u: usize,
            present: &[i32],
            future: &[i32],
            g: &[Vec<usize>],
            budget: usize,
        ) -> (Vec<i32>, Vec<i32>, usize) {
            let cost = present[u] as usize;
            let d_cost = present[u] as usize / 2;

            // dp[u][state][budget]
            // state = 0: Do not purchase parent node, state = 1: Must purchase parent node
            let mut dp0 = vec![0; budget + 1];
            let mut dp1 = vec![0; budget + 1];

            // subProfit[state][budget]
            // state = 0: discount not available, state = 1: discount available
            let mut sub_profit0 = vec![0; budget + 1];
            let mut sub_profit1 = vec![0; budget + 1];
            let mut u_size = cost;

            for &v in &g[u] {
                let (child_dp0, child_dp1, v_size) = dfs(v, present, future, g, budget);
                u_size += v_size;

                for i in (0..=budget).rev() {
                    for sub in 0..=v_size.min(i) {
                        if i >= sub {
                            sub_profit0[i] =
                                sub_profit0[i].max(sub_profit0[i - sub] + child_dp0[sub]);
                            sub_profit1[i] =
                                sub_profit1[i].max(sub_profit1[i - sub] + child_dp1[sub]);
                        }
                    }
                }
            }

            for i in 0..=budget {
                dp0[i] = sub_profit0[i];
                dp1[i] = sub_profit0[i];
                if i >= d_cost {
                    dp1[i] = dp1[i].max(sub_profit1[i - d_cost] + future[u] - d_cost as i32);
                }
                if i >= cost {
                    dp0[i] = dp0[i].max(sub_profit1[i - cost] + future[u] - cost as i32);
                }
            }

            (dp0, dp1, u_size)
        }

        let (dp0, _, _) = dfs(0, &present, &future, &g, budget);
        dp0[budget]
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3562() {
        assert_eq!(
            Solution::max_profit(2, vec![1, 2], vec![4, 3], vec![vec![1, 2]], 3),
            5
        );
        assert_eq!(
            Solution::max_profit(2, vec![3, 4], vec![5, 8], vec![vec![1, 2]], 4),
            4
        );
        assert_eq!(
            Solution::max_profit(
                3,
                vec![4, 6, 8],
                vec![7, 9, 11],
                vec![vec![1, 2], vec![1, 3]],
                10
            ),
            10
        );
        assert_eq!(
            Solution::max_profit(
                3,
                vec![5, 2, 3],
                vec![8, 5, 6],
                vec![vec![1, 2], vec![2, 3]],
                7
            ),
            11
        );
    }
}
