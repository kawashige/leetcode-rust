pub struct Solution {}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let mut max_count = 1;

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let count = 2
                    + (j + 1..points.len())
                        .filter(|k| {
                            (points[*k][0] - points[i][0]) * (points[j][1] - points[i][1])
                                == (points[j][0] - points[i][0]) * (points[*k][1] - points[i][1])
                        })
                        .count();
                max_count = max_count.max(count);
            }
        }

        max_count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1049() {
        assert_eq!(Solution::max_points(vec![vec![0, 0]]), 1);
        assert_eq!(Solution::max_points(vec![vec![0, 0], vec![1, 1]]), 2);
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
    }
}
