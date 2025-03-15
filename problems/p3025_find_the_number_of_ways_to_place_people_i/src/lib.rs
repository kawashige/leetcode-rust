pub struct Solution {}

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by_key(|a| (a[0], -a[1]));

        let mut result = 0;
        for i in 0..points.len() {
            for j in 0..i {
                if points[j][1] >= points[i][1]
                    && (0..points.len()).all(|k| {
                        k == i
                            || k == j
                            || !((points[j][0]..=points[i][0]).contains(&points[k][0])
                                && (points[j][1].min(points[i][1])
                                    ..=points[i][1].max(points[j][1]))
                                    .contains(&points[k][1]))
                    })
                {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3025() {
        assert_eq!(
            Solution::number_of_pairs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            0
        );
        assert_eq!(
            Solution::number_of_pairs(vec![vec![6, 2], vec![4, 4], vec![2, 6]]),
            2
        );
        assert_eq!(
            Solution::number_of_pairs(vec![vec![3, 1], vec![1, 3], vec![1, 1]]),
            2
        );
    }
}
