pub struct Solution {}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut start_points = Vec::new();
        let mut end_points = Vec::new();
        for (i, interval) in intervals.iter().enumerate() {
            start_points.push(vec![interval[0], i as i32]);
            end_points.push(interval[1]);
        }
        start_points.sort();
        end_points
            .iter()
            .map(|s| match start_points.binary_search_by(|v| v[0].cmp(&s)) {
                Ok(i) => start_points[i][1],
                Err(i) => {
                    if i < start_points.len() {
                        start_points[i][1]
                    } else {
                        -1
                    }
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day27() {
        assert_eq!(vec![-1], Solution::find_right_interval(vec![vec![1, 2]]));
        assert_eq!(
            vec![-1, 0, 1],
            Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]])
        );
        assert_eq!(
            vec![-1, 2, -1],
            Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]])
        );
    }
}
