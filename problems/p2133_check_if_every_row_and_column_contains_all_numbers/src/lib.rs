pub struct Solution {}

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        for i in 0..matrix.len() {
            let mut row = vec![false; matrix.len() + 1];
            let mut column = vec![false; matrix.len() + 1];
            for j in 0..matrix.len() {
                if row[matrix[i][j] as usize] {
                    return false;
                }
                if column[matrix[j][i] as usize] {
                    return false;
                }
                row[matrix[i][j] as usize] = true;
                column[matrix[j][i] as usize] = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2133() {
        assert!(Solution::check_valid(vec![
            vec![1, 2, 3],
            vec![3, 1, 2],
            vec![2, 3, 1]
        ]));
        assert!(!Solution::check_valid(vec![
            vec![1, 1, 1],
            vec![1, 2, 3],
            vec![1, 2, 3]
        ]));
    }
}
