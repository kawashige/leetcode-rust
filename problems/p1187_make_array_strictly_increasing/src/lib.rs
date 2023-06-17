use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let arr2 = arr2
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let mut dp = vec![vec![std::i32::MAX; arr2.len() + 1]; arr1.len()];

        for i in 0..arr2.len() {
            dp[0][i] = 1;
        }
        dp[0][arr2.len()] = 0;

        for i in 1..arr1.len() {
            let mut min_tmp = std::i32::MAX;
            for j in 0..arr2.len() {
                if arr2[j] < arr1[i] {
                    dp[i][arr2.len()] = dp[i][arr2.len()].min(dp[i - 1][j]);
                }
                if arr1[i - 1] < arr2[j] && dp[i - 1][arr2.len()] != std::i32::MAX {
                    dp[i][j] = dp[i][j].min(dp[i - 1][arr2.len()] + 1)
                }
                if min_tmp != std::i32::MAX {
                    dp[i][j] = dp[i][j].min(min_tmp + 1);
                }
                min_tmp = min_tmp.min(dp[i - 1][j]);
            }
            if arr1[i - 1] < arr1[i] {
                dp[i][arr2.len()] = dp[i][arr2.len()].min(dp[i - 1][arr2.len()]);
            }
        }

        let min_count = *dp[arr1.len() - 1].iter().min().unwrap();
        if min_count == std::i32::MAX {
            -1
        } else {
            min_count
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1187() {
        // assert_eq!(
        //     Solution::make_array_increasing(
        //         vec![9, 18, 3, 8, 21, 6, 7, 2, 7, 28, 23, 16, 33, 2, 25, 14, 15],
        //         vec![
        //             13, 2, 15, 30, 31, 30, 9, 10, 7, 30, 31, 4, 33, 10, 25, 28, 19, 6, 15, 6, 19,
        //             30, 25, 14, 7, 28, 23, 20, 1, 2, 25, 16
        //         ]
        //     ),
        //     17
        // );
        assert_eq!(
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]),
            1
        );
        assert_eq!(
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]),
            2
        );
        assert_eq!(
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]),
            -1
        );
    }
}
