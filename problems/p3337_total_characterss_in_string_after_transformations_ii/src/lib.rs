pub struct Solution {}

const M: usize = 1_000_000_007;

impl Solution {
    pub fn matrix_mul(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut result = vec![vec![0; b[0].len()]; a.len()];
        for i in 0..a.len() {
            for j in 0..b[0].len() {
                result[i][j] = (0..b.len()).fold(0, |sum, k| (sum + (a[i][k] * b[k][j]) % M) % M)
            }
        }
        result
    }

    pub fn matrix_pow(mat: Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
        let mut n = n;
        let mut result = vec![vec![0; mat.len()]; mat.len()];
        for i in 0..mat.len() {
            result[i][i] = 1;
        }
        let mut tmp = mat;

        while 0 < n {
            if n % 2 == 1 {
                result = Self::matrix_mul(&result, &tmp);
            }
            n /= 2;
            tmp = Self::matrix_mul(&tmp, &tmp);
        }

        result
    }

    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut mat = vec![vec![0; 26]; s.len()];
        for i in 0..s.len() {
            mat[i][(s.as_bytes()[i] - b'a') as usize] = 1;
        }
        let mut tran = vec![vec![0; 26]; 26];
        for i in 0..26 {
            for j in i + 1..i + 1 + nums[i] as usize {
                tran[i][j % 26] = 1;
            }
        }

        mat = Self::matrix_mul(&mat, &Self::matrix_pow(tran, t as usize));
        let mut result = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                result += mat[i][j];
                result %= M;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3337() {
        assert_eq!(
            Solution::length_after_transformations(
                "k".to_string(),
                2,
                vec![2, 2, 1, 3, 1, 1, 2, 3, 3, 2, 1, 2, 2, 1, 1, 3, 1, 2, 2, 1, 3, 3, 3, 2, 2, 1],
            ),
            2
        );
        assert_eq!(
            Solution::length_after_transformations(
                "abcyy".to_string(),
                2,
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
            ),
            7
        );
        assert_eq!(
            Solution::length_after_transformations(
                "azbk".to_string(),
                1,
                vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
            ),
            8
        );
    }
}
