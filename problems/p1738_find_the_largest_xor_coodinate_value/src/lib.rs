pub struct Solution {}

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut acc = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut xor_values = Vec::with_capacity(matrix.len() * matrix[0].len());

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                acc[i][j] = matrix[i][j];
                if 0 < i {
                    acc[i][j] ^= acc[i - 1][j];
                }
                if 0 < j {
                    acc[i][j] ^= acc[i][j - 1];
                }
                if 0 < i && 0 < j {
                    acc[i][j] ^= acc[i - 1][j - 1];
                }
                xor_values.push(acc[i][j]);
            }
        }

        println!("{:?}", xor_values);

        xor_values.sort_unstable_by(|a, b| b.cmp(&a));
        xor_values[k as usize - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1738() {
        assert_eq!(
            Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1),
            7
        );
        assert_eq!(
            Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2),
            5
        );
        assert_eq!(
            Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3),
            4
        );
    }
}
