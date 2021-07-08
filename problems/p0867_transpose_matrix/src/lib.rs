pub struct Solution {}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        (0..matrix[0].len())
            .map(|j| {
                (0..matrix.len())
                    .map(|i| matrix[i][j])
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0867() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }
}
