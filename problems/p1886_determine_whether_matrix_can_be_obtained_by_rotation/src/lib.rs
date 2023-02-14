pub struct Solution {}

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut mat = mat;
        for _ in 0..4 {
            if (0..mat.len()).all(|i| (0..mat.len()).all(|j| mat[i][j] == target[i][j])) {
                return true;
            }
            let mut new_mat = vec![vec![0; mat.len()]; mat.len()];
            for i in 0..mat.len() {
                for j in 0..mat[0].len() {
                    new_mat[j][mat.len() - 1 - i] = mat[i][j];
                }
            }
            mat = new_mat;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1866() {
        assert!(Solution::find_rotation(
            vec![vec![0, 1], vec![1, 0]],
            vec![vec![1, 0], vec![0, 1]]
        ));
        assert!(!Solution::find_rotation(
            vec![vec![0, 1], vec![1, 1]],
            vec![vec![1, 0], vec![0, 1]]
        ));
        assert!(Solution::find_rotation(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]
        ));
    }
}
