pub struct Solution {}

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));

        let mut result = intervals.len() as i32;
        let mut i = 0;
        while i < intervals.len() - 1 {
            let mut j = i + 1;
            while j < intervals.len()
                && intervals[i][0] <= intervals[j][0]
                && intervals[j][1] <= intervals[i][1]
            {
                result -= 1;
                j += 1;
            }
            i = j;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day4() {
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]])
        );
        assert_eq!(
            1,
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]])
        );
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![0, 10], vec![5, 11]])
        );
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![3, 10], vec![4, 10], vec![5, 11]])
        );
        assert_eq!(
            1,
            Solution::remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]])
        );
    }
}
