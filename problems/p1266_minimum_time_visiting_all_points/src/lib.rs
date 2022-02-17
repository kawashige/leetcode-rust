pub struct Solution {}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|p| (p[0][0] - p[1][0]).abs().max((p[0][1] - p[1][1]).abs()))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1266() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
    }
}
