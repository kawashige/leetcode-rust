pub struct Solution {}

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last_day = (days.last().unwrap_or(&0) + 1) as usize;
        let mut dp = vec![0; last_day];
        for i in 1..last_day {
            if days.contains(&(i as i32)) {
                dp[i] = *vec![
                    dp[i.saturating_sub(1)] + costs[0],
                    dp[i.saturating_sub(7)] + costs[1],
                    dp[i.saturating_sub(30)] + costs[2],
                ]
                .iter()
                .min()
                .unwrap();
            } else {
                dp[i] = dp[i - 1];
            }
        }
        println!("{:?}", dp);
        *dp.last().unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day25() {
        assert_eq!(
            11,
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15])
        );
        assert_eq!(
            17,
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15])
        );
        assert_eq!(
            17,
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15])
        );
        assert_eq!(
            35,
            Solution::mincost_tickets(
                vec![1, 4, 6, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 27, 28],
                vec![3, 13, 35]
            )
        );
        assert_eq!(
            44,
            Solution::mincost_tickets(
                vec![1, 4, 6, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 27, 28],
                vec![3, 13, 45]
            )
        );
    }
}
