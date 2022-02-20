pub struct Solution {}

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut sums = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut result = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                sums[i][j] += matrix[i][j] as usize;
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

                    if (k + 1) * (k + 1) == area {
                        result += 1;
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
    fn test_1277() {
        assert_eq!(
            Solution::count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]]),
            15
        );
        assert_eq!(
            Solution::count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            7
        );
    }
}
