pub struct Solution {}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let mut row = points[0].iter().map(|p| *p as i64).collect::<Vec<_>>();

        for i in 1..points.len() {
            let mut max_value = 0;
            let mut new_row = vec![0_i64; points[0].len()];
            for j in 0..points[0].len() {
                max_value = (max_value - 1).max(row[j]);
                new_row[j] = max_value;
            }
            max_value = 0;
            for j in (0..points[0].len()).rev() {
                max_value = (max_value - 1).max(row[j]);
                new_row[j] = new_row[j].max(max_value);
                new_row[j] += points[i][j] as i64;
            }
            row = new_row;
        }

        row.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1937() {
        assert_eq!(
            Solution::max_points(vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]]),
            9
        );
        assert_eq!(
            Solution::max_points(vec![vec![1, 5], vec![2, 3], vec![4, 2]]),
            11
        );
    }
}
