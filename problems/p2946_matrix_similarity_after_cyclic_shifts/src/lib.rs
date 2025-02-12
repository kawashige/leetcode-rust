pub struct Solution {}

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if i % 2 == 0 {
                    if mat[i][j]
                        != mat[i][(j + mat[0].len() - (k as usize % mat[0].len())) % mat[0].len()]
                    {
                        return false;
                    }
                } else {
                    if mat[i][j] != mat[i][(j + k as usize % mat[0].len()) % mat[0].len()] {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2946() {
        assert!(!Solution::are_similar(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            4
        ));
        assert!(Solution::are_similar(
            vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]],
            2
        ));
        assert!(Solution::are_similar(vec![vec![2, 2], vec![2, 2]], 3));
    }
}
