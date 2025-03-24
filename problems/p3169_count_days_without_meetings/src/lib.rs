pub struct Solution {}

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        meetings.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let mut result = meetings[0][0] - 1;
        let mut t = meetings[0][1];

        for i in 1..meetings.len() {
            if meetings[i][0] <= t {
                t = meetings[i][1].max(t);
            } else {
                result += meetings[i][0] - t - 1;
                t = meetings[i][1];
            }
        }

        result + days - t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3169() {
        assert_eq!(
            Solution::count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]),
            2
        );
        assert_eq!(Solution::count_days(5, vec![vec![2, 4], vec![1, 3]]), 1);
        assert_eq!(Solution::count_days(6, vec![vec![1, 6]]), 0);
    }
}
