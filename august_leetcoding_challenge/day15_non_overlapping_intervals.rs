pub struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        if intervals.len() > 0 {
            let mut sorted_intervals = intervals.clone();
            sorted_intervals.sort_by_key(|i| i[0]);
            let mut tmp = sorted_intervals[0].clone();
            for i in 1..sorted_intervals.len() {
                if tmp[1] > sorted_intervals[i][0] {
                    if tmp[1] > sorted_intervals[i][1] {
                        tmp = sorted_intervals[i].clone();
                    }
                    result += 1;
                } else {
                    tmp = sorted_intervals[i].clone();
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day15() {
        assert_eq!(
            1,
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]])
        );
        assert_eq!(
            0,
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]])
        );
        assert_eq!(
            1,
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4]])
        );
        assert_eq!(
            2,
            Solution::erase_overlap_intervals(vec![vec![1, 3], vec![2, 9], vec![2, 4]])
        );
        assert_eq!(0, Solution::erase_overlap_intervals(vec![]));
        assert_eq!(
            2,
            Solution::erase_overlap_intervals(vec![
                vec![0, 2],
                vec![1, 3],
                vec![2, 4],
                vec![3, 5],
                vec![4, 6]
            ])
        );
    }
}
