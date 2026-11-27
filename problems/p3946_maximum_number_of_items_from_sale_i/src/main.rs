pub struct Solution {}

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let mut copies = vec![1; items.len()];
        for i in 0..items.len() {
            for j in 0..items.len() {
                if i == j || items[j][0] % items[i][0] != 0 {
                    continue;
                }
                copies[i] += 1;
            }
        }
        let min = items.iter().map(|i| i[1]).min().unwrap();

        let mut dp = vec![vec![0; budget as usize + 1]; items.len() + 1];
        for i in 0..items.len() {
            for j in 0..dp[i].len() {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                if dp[i].len() <= items[i][1] as usize + j {
                    continue;
                }
                dp[i + 1][items[i][1] as usize + j] =
                    dp[i + 1][items[i][1] as usize + j].max(dp[i][j] + copies[i]);
            }
        }

        let mut result = 0;
        for i in 0..dp[0].len() {
            result = result.max(dp[dp.len() - 1][i] + (budget - i as i32) / min);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3946() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![6, 2], vec![2, 6], vec![3, 4]], 9),
            4
        );
        assert_eq!(
            Solution::maximum_sale_items(
                vec![vec![2, 4], vec![3, 2], vec![4, 1], vec![6, 4], vec![12, 4]],
                8
            ),
            10
        );
    }
}

fn main() {}
