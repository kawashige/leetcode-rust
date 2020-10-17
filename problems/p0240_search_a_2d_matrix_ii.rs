pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let i = (0..matrix.len()).filter(|i| matrix[*i][0] <= target).max();
        if i.is_none() {
            return false;
        }
        let mut j: usize = 0;
        for k in (0..=i.unwrap()).rev() {
            while j < matrix[0].len() - 1 && matrix[k][j + 1] <= target {
                j += 1;
            }
            if matrix[k][j] == target {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0240() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(Solution::search_matrix(matrix.clone(), 5));
        assert!(!Solution::search_matrix(matrix, 20));
        assert!(!Solution::search_matrix(Vec::new() as Vec<Vec<i32>>, 0));
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        assert!(Solution::search_matrix(matrix, 5));
        assert!(!Solution::search_matrix(vec![vec![]] as Vec<Vec<i32>>, 0));
        assert!(!Solution::search_matrix(vec![vec![-5]], -10));
        assert!(Solution::search_matrix(vec![vec![-5]], -5));
        assert!(Solution::search_matrix(
            vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25]
            ],
            19
        ));
        assert!(Solution::search_matrix(
            vec![
                vec![1, 3, 5, 7, 9],
                vec![2, 4, 6, 8, 10],
                vec![11, 13, 15, 17, 19],
                vec![12, 14, 16, 18, 20],
                vec![21, 22, 23, 24, 25]
            ],
            13
        ))
    }
}
