pub struct Solution {}

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut accum = vec![vec![0; mat[0].len()]; mat.len()];

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                accum[i][j] += mat[i][j];
                if 0 < i {
                    accum[i][j] += accum[i - 1][j];
                }
                if 0 < j {
                    accum[i][j] += accum[i][j - 1];
                }
                if 0 < i && 0 < j {
                    accum[i][j] -= accum[i - 1][j - 1];
                }
            }
        }

        let mut result = vec![vec![0; mat[0].len()]; mat.len()];

        for i in 0..result.len() {
            for j in 0..result[0].len() {
                let e_i = (i + k as usize).min(accum.len() - 1);
                let e_j = (j + k as usize).min(accum[0].len() - 1);
                result[i][j] = accum[e_i][e_j];
                if k < i as i32 {
                    result[i][j] -= accum[i - k as usize - 1][e_j];
                }
                if k < j as i32 {
                    result[i][j] -= accum[e_i][j - k as usize - 1];
                }
                if k < i as i32 && k < j as i32 {
                    result[i][j] += accum[i - k as usize - 1][j - k as usize - 1];
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
    fn test_1314() {
        assert_eq!(
            Solution::matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]]
        );
        assert_eq!(
            Solution::matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 2),
            vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]]
        );
    }
}
