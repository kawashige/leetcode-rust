pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if intervals.len() > 0 {
            let mut sorted_intervals = intervals.clone();
            sorted_intervals.sort_by_key(|i| i[0]);
            let mut tmp = sorted_intervals[0].clone();
            for next in sorted_intervals[1..].to_vec() {
                if next[0] <= tmp[1] {
                    tmp = vec![
                        std::cmp::min(tmp[0], next[0]),
                        std::cmp::max(tmp[1], next[1]),
                    ];
                } else {
                    result.push(tmp);
                    tmp = next;
                }
            }
            result.push(tmp);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0056() {
        assert_eq!(
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        );
        assert_eq!(
            vec![vec![1, 5]],
            Solution::merge(vec![vec![1, 4], vec![4, 5]])
        );
        assert_eq!(vec![vec![1, 5]], Solution::merge(vec![vec![1, 5]]));
        assert_eq!(
            vec![vec![1, 18]],
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![2, 10], vec![1, 18]])
        );
        let result: Vec<Vec<i32>> = Vec::new();
        assert_eq!(result, Solution::merge(Vec::new()));
        assert_eq!(
            vec![vec![0, 0], vec![1, 4]],
            Solution::merge(vec![vec![1, 4], vec![0, 0]])
        );
    }
}
