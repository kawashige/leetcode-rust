pub struct Solution {}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = (0..start_time.len())
            .map(|i| (start_time[i], end_time[i], profit[i]))
            .collect::<Vec<_>>();
        jobs.sort_unstable();

        let mut dp = vec![-1; jobs.len()];

        for i in (0..jobs.len()).rev() {
            let (_, e, p) = jobs[i];

            let mut j = jobs
                .binary_search_by_key(&e, |j| j.0)
                .map_or_else(|j| j, |j| j);
            while 0 < j && j < jobs.len() && jobs[j].0 == jobs[j - 1].0 {
                j -= 1;
            }

            dp[i] = p;
            if i < jobs.len() - 1 {
                dp[i] = dp[i].max(dp[i + 1]);
            }
            if j < jobs.len() {
                dp[i] = dp[i].max(dp[j] + p);
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day28() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );
        assert_eq!(
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6
        );
    }
}
