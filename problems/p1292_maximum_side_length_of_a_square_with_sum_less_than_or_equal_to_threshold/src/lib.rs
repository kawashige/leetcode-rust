pub struct Solution {}

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let mut sums = vec![vec![0; mat[0].len()]; mat.len()];
        let mut result = 0;

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                sums[i][j] += mat[i][j] as usize;
                if i > 0 {
                    sums[i][j] += sums[i - 1][j];
                }
                if j > 0 {
                    sums[i][j] += sums[i][j - 1];
                }
                if i > 0 && j > 0 {
                    sums[i][j] -= sums[i - 1][j - 1];
                }

                for k in 0..=i.min(j) {
                    let area = sums[i][j]
                        + if i > k && j > k {
                            sums[i - k - 1][j - k - 1]
                        } else {
                            0
                        }
                        - if i > k { sums[i - k - 1][j] } else { 0 }
                        - if j > k { sums[i][j - k - 1] } else { 0 };

                    if area <= threshold as usize {
                        result = result.max(k as i32 + 1);
                    }
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
    fn test_1292() {
        assert_eq!(
            Solution::max_side_length(
                vec![
                    vec![1, 1, 3, 2, 4, 3, 2],
                    vec![1, 1, 3, 2, 4, 3, 2],
                    vec![1, 1, 3, 2, 4, 3, 2]
                ],
                4
            ),
            2
        );
        assert_eq!(
            Solution::max_side_length(
                vec![
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2]
                ],
                1
            ),
            0
        );
    }
}
