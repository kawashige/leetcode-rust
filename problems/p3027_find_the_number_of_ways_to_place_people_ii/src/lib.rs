pub struct Solution {}

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let mut result = 0;

        for i in 0..points.len() {
            let mut lower = std::i32::MAX;
            for j in (0..i).rev() {
                if points[j][1] < points[i][1] || (lower != std::i32::MAX && lower <= points[j][1])
                {
                    continue;
                }
                result += 1;
                lower = points[j][1];
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3027() {
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
