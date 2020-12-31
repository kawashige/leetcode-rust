pub struct Solution {}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let counts = strs
            .iter()
            .map(|s| {
                let ones = u128::from_str_radix(s, 2).unwrap().count_ones() as usize;
                let zeros = s.len() - ones;
                (zeros, ones)
            })
            .collect::<Vec<(usize, usize)>>();

        println!("counts: {:?}", counts);
        let mut dp = vec![vec![vec![0_usize; n + 1]; m + 1]; counts.len() + 1];
        println!("dp.len(): {:?}, counts.len(): {:?}", dp.len(), counts.len());
        for i in 0..counts.len() {
            println!("dp[i]: {:?}, counts[i[: {:?}", dp[i], counts[i]);
            for j in 0..=m {
                for k in 0..=n {
                    if counts[i].0 <= j && counts[i].1 <= k {
                        dp[i + 1][j][k] =
                            std::cmp::max(dp[i][j][k], dp[i][j - counts[i].0][k - counts[i].1] + 1);
                    } else {
                        dp[i + 1][j][k] = dp[i][j][k];
                    }
                }
            }
        }
        println!("dp.last(): {:?}", dp[dp.len() - 1]);

        dp[dp.len() - 1][m][n] as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0474() {
        assert_eq!(
            4,
            Solution::find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            )
        );
        assert_eq!(
            2,
            Solution::find_max_form(
                vec!["10".to_string(), "1".to_string(), "0".to_string()],
                1,
                1
            )
        );
    }
}
