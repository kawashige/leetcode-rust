pub struct Solution {}

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut i = -1;
        let mut d = std::i32::MAX;

        for j in 0..points.len() {
            if points[j][0] != x && points[j][1] != y {
                continue;
            }
            let new_d = (points[j][0] - x).abs() + (points[j][1] - y).abs();
            if new_d < d {
                i = j as i32;
                d = new_d;
            }
        }

        i
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1779() {
        assert_eq!(
            Solution::nearest_valid_point(
                3,
                4,
                vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]
            ),
            2
        );
        assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]), 0);
        assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]), -1);
    }
}
