pub struct Solution {}

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut d_p = difficulty
            .into_iter()
            .zip(profit.into_iter())
            .collect::<Vec<(i32, i32)>>();
        d_p.sort_unstable();
        worker.sort_unstable();

        let mut max = 0;
        let mut profit = 0;
        let mut j = 0;
        for i in 0..worker.len() {
            while j < d_p.len() && d_p[j].0 <= worker[i] {
                max = std::cmp::max(max, d_p[j].1);
                j += 1;
            }
            profit += max;
        }

        profit
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0826() {
        assert_eq!(
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            ),
            100
        );
    }
}
