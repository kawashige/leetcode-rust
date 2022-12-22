pub struct Solution {}

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|query| {
                points
                    .iter()
                    .filter(|point| {
                        (query[0] - point[0]).pow(2) + (query[1] - point[1]).pow(2)
                            <= query[2].pow(2)
                    })
                    .count() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1828() {
        assert_eq!(
            Solution::count_points(
                vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
                vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]]
            ),
            vec![3, 2, 2]
        );
        assert_eq!(
            Solution::count_points(
                vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
                vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]]
            ),
            vec![2, 3, 2, 4]
        );
    }
}
