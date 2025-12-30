pub struct Solution {}

impl Solution {
    pub fn maximum_coins(coins: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut coins = coins;
        coins.sort_unstable_by_key(|c| c[0]);

        let mut result = 0;
        let mut j = 0;
        let mut sum1 = coins[0][2] as i64 * (coins[0][1] - coins[0][0] + 1) as i64;
        let mut l = 0;
        let mut sum2 = 0;

        for i in 0..coins.len() {
            if j < i {
                j = i;
                sum1 = coins[i][2] as i64 * (coins[i][1] - coins[i][0] + 1) as i64;
            } else if 0 < i {
                sum1 -= coins[i - 1][2] as i64 * (coins[i - 1][1] - coins[i - 1][0] + 1) as i64;
            }
            while j + 1 < coins.len() && coins[j + 1][0] <= coins[i][0] + k - 1 {
                j += 1;
                sum1 += coins[j][2] as i64 * (coins[j][1] - coins[j][0] + 1) as i64;
            }
            result = result.max(
                sum1 - if coins[i][0] + k - 1 < coins[j][1] {
                    coins[j][2] as i64 * (coins[j][1] - (coins[i][0] + k - 1)) as i64
                } else {
                    0
                },
            );

            sum2 += coins[i][2] as i64 * (coins[i][1] - coins[i][0] + 1) as i64;
            while l + 1 <= i && coins[l][1] < coins[i][1] - k + 1 {
                sum2 -= coins[l][2] as i64 * (coins[l][1] - coins[l][0] + 1) as i64;
                l += 1;
            }
            result = result.max(
                sum2 - if coins[l][0] < coins[i][1] - k + 1 {
                    coins[l][2] as i64 * (coins[i][1] - k + 1 - coins[l][0]) as i64
                } else {
                    0
                },
            );
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3413() {
        assert_eq!(
            Solution::maximum_coins(vec![vec![8, 10, 1], vec![1, 3, 2], vec![5, 6, 4]], 4),
            10
        );
        assert_eq!(Solution::maximum_coins(vec![vec![1, 10, 3]], 2), 6);
    }
}
