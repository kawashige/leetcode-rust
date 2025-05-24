pub struct Solution {}

impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        let mut points = points;
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut result = 0;
        let mut l = -1;

        for i in 0..points.len() {
            if l < points[i][0] {
                result += 1;
                l = points[i][0] + w;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3111() {
        assert_eq!(
            Solution::min_rectangles_to_cover_points(
                vec![
                    vec![2, 1],
                    vec![1, 0],
                    vec![1, 4],
                    vec![1, 8],
                    vec![3, 5],
                    vec![4, 6]
                ],
                1
            ),
            2
        );
        assert_eq!(
            Solution::min_rectangles_to_cover_points(
                vec![
                    vec![0, 0],
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 3],
                    vec![4, 4],
                    vec![5, 5],
                    vec![6, 6]
                ],
                2
            ),
            3
        );
        assert_eq!(
            Solution::min_rectangles_to_cover_points(vec![vec![2, 3], vec![1, 2]], 0),
            2
        );
    }
}
